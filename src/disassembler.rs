use std::fs;
use std::io;

use crate::bytecode::read_bytecode;
use crate::isa::Op;

pub fn disassemble_file(input: &str, output: &str) -> io::Result<()> {
    let code = read_bytecode(input)?;

    let text = disassemble(&code)?;

    fs::write(output, text)
}

fn disassemble(code: &[u8]) -> io::Result<String> {
    let mut pc = 0;
    let mut output = String::new();

    while pc < code.len() {
        let (op, size) =
            Op::decode(&code[pc..])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        pc += size;

        match op {
            Op::Push(v) => output.push_str(&format!("PUSH {}\n", v)),
            Op::Pop => output.push_str("POP\n"),
            Op::Dup => output.push_str("DUP\n"),
            Op::Swap => output.push_str("SWAP\n"),

            Op::Add => output.push_str("ADD\n"),
            Op::Sub => output.push_str("SUB\n"),
            Op::Mul => output.push_str("MUL\n"),
            Op::Div => output.push_str("DIV\n"),
            Op::Mod => output.push_str("MOD\n"),
            Op::Neg => output.push_str("NEG\n"),

            Op::Load(slot) => {
                output.push_str(&format!("LOAD {}\n", slot));
            }

            Op::Store(slot) => {
                output.push_str(&format!("STORE {}\n", slot));
            }

            Op::Print => output.push_str("PRINT\n"),
            Op::Halt => output.push_str("HALT\n"),
        }
    }

    Ok(output)
}