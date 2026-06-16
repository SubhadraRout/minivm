use std::io;

use crate::bytecode::read_bytecode;
use crate::isa::Op;

const MAX_STACK: usize = 1024;

pub struct Vm {
    stack: Vec<i64>,
    globals: [i64; 256],
    pc: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            globals: [0; 256],
            pc: 0,
        }
    }

    pub fn run_file(path: &str, trace: bool) -> io::Result<()> {
        let code = read_bytecode(path)?;

        let mut vm = Vm::new();

        vm.execute(&code, trace)
    }

    fn push(&mut self, value: i64) -> io::Result<()> {
        if self.stack.len() >= MAX_STACK {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "trap at ip=0x{:04X}: stack overflow",
                    self.pc
                ),
            ));
        }

        self.stack.push(value);

        Ok(())
    }

    fn pop(&mut self) -> io::Result<i64> {
        self.stack.pop().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "trap at ip=0x{:04X}: stack underflow",
                    self.pc
                ),
            )
        })
    }

    fn execute(&mut self, code: &[u8], trace: bool) -> io::Result<()> {
        self.pc = 0;

        while self.pc < code.len() {
            let (op, size) =
                Op::decode(&code[self.pc..])
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            if trace {
                println!(
                    "ip=0x{:04X} {:?} stack={:?}",
                    self.pc,
                    op,
                    self.stack
                );
            }

            self.pc += size;

            match op {
                Op::Push(value) => {
                    self.push(value)?;
                }

                Op::Pop => {
                    self.pop()?;
                }

                Op::Dup => {
                    let value = *self
                        .stack
                        .last()
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!(
                                    "trap at ip=0x{:04X}: stack underflow",
                                    self.pc
                                ),
                            )
                        })?;

                    self.push(value)?;
                }

                Op::Swap => {
                    if self.stack.len() < 2 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "trap at ip=0x{:04X}: stack underflow",
                                self.pc
                            )
                        ));
                    }

                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                }

                Op::Halt => {
                    return Ok(());
                }

               Op::Add => {
                    let b = self.pop()?;

                    let a = self.pop()?;

                    self.push(a + b)?;
                }

                Op::Sub => {
                    let b = self.pop()?;

                    let a = self.pop()?;

                    self.push(a - b)?;
                }


                Op::Mul => {
                    let b = self.pop()?;

                    let a = self.pop()?;
                    

                    self.push(a * b)?;
                }
            

                Op::Div => {
                    let b = self.pop()?;

                    if b == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "trap at ip=0x{:04X}: division by zero",
                                self.pc
                            )
                        ));
                    }

                    let a = self.pop()?;

                    self.push(a / b)?;
                }

                Op::Mod => {
                    let b = self.pop()?;

                    if b == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "trap at ip=0x{:04X}: modulo by zero",
                                self.pc
                            )
                        ));
                    }

                    let a = self.pop()?;

                    self.push(a % b)?;
                }

                Op::Neg => {
                    let value = self.pop()?;

                    self.push(-value)?;
                }

                Op::Load(slot) => {
                    let value = self.globals[slot as usize];
                    self.push(value)?;
                }

                Op::Store(slot) => {
                    let value = self.pop()?;

                    self.globals[slot as usize] = value;
                }

                Op::Print => {
                    let value = self.pop()?;
                    println!("{}", value);
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "trap at ip=0x{:04X}: program ended without HALT",
                self.pc
            ),
        ))
    }
}