use std::io;

use crate::bytecode::read_bytecode;
use crate::isa::Op;

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

    pub fn run_file(path: &str) -> io::Result<()> {
        let code = read_bytecode(path)?;

        let mut vm = Vm::new();

        vm.execute(&code)
    }

    fn execute(&mut self, code: &[u8]) -> io::Result<()> {
        self.pc = 0;

        while self.pc < code.len() {
            let (op, size) =
                Op::decode(&code[self.pc..])
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            self.pc += size;

            match op {
                Op::Push(value) => {
                    self.stack.push(value);
                }

                Op::Pop => {
                    if self.stack.pop().is_none() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Stack underflow",
                        ));
                    }
                }

                Op::Dup => {
                    let value = *self
                        .stack
                        .last()
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Stack underflow",
                            )
                        })?;

                    self.stack.push(value);
                }

                Op::Swap => {
                    if self.stack.len() < 2 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Stack underflow",
                        ));
                    }

                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                }

                Op::Halt => {
                    break;
                }

               Op::Add => {
                    let b = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    let a = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    self.stack.push(a + b);
                }

                Op::Sub => {
                    let b = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    let a = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    self.stack.push(a - b);
                }

                Op::Mul => {
                    let b = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    let a = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    self.stack.push(a * b);
                }

                Op::Div => {
                    let b = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    if b == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Division by zero",
                        ));
                    }

                    let a = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    self.stack.push(a / b);
                }

                Op::Mod => {
                    let b = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    if b == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Modulo by zero",
                        ));
                    }

                    let a = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    self.stack.push(a % b);
                }

                Op::Neg => {
                    let value = self.stack.pop().ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidData, "Stack underflow")
                    })?;

                    self.stack.push(-value);
                }

                Op::Load(slot) => {
                    let value = self.globals[slot as usize];
                    self.stack.push(value);
                }

                Op::Store(slot) => {
                    let value = self.stack.pop().ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Stack underflow",
                        )
                    })?;

                    self.globals[slot as usize] = value;
                }

                Op::Print => {
                    let value = self.stack.last().ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Stack underflow",
                        )
                    })?;

                    println!("{}", value);
                }
            }
        }

        Ok(())
    }
}