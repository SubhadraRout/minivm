use std::fs;
use std::io::{self, Error, ErrorKind};

use crate::bytecode::write_bytecode;
use crate::isa::Op;

/// Assemble a .tasm file into a .tbc file.
pub fn assemble_file(input: &str, output: &str) -> io::Result<()> {
    let source = fs::read_to_string(input)?;

    let program = parse_program(&source)?;

    if !matches!(program.last(), Some(Op::Halt)) {
        eprintln!("warning: program does not end with HALT");
    }

    let code = encode_program(&program);

    write_bytecode(output, &code)
}

/// Parse an entire assembly program.
fn parse_program(source: &str) -> io::Result<Vec<Op>> {
    let mut program = Vec::new();

    for (idx, raw_line) in source.lines().enumerate() {
        let line_no = idx + 1;

        let line = strip_comments(raw_line);

        if line.is_empty() {
            continue;
        }

        let op = parse_instruction(line, line_no)?;

        program.push(op);
    }

    Ok(program)
}

/// Remove ';' comments and trim whitespace.
fn strip_comments(line: &str) -> &str {
    line.split(';')
        .next()
        .unwrap()
        .trim()
}

/// Parse one instruction.
fn parse_instruction(line: &str, line_no: usize) -> io::Result<Op> {

    let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Line {}: empty instruction", line_no),
        ));
    }

    let mnemonic = tokens[0].to_ascii_uppercase();

    match mnemonic.as_str() {

        "PUSH" => {
            expect_operands(&tokens, 2, line_no)?;

            let value = parse_i64(tokens[1], line_no)?;

            Ok(Op::Push(value))
        }

        "POP" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Pop)
        }

        "DUP" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Dup)
        }

        "SWAP" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Swap)
        }

        "ADD" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Add)
        }

        "SUB" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Sub)
        }

        "MUL" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Mul)
        }

        "DIV" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Div)
        }

        "MOD" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Mod)
        }

        "NEG" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Neg)
        }

        "LOAD" => {
            expect_operands(&tokens, 2, line_no)?;

            let slot = parse_u8(tokens[1], line_no)?;

            Ok(Op::Load(slot))
        }

        "STORE" => {
            expect_operands(&tokens, 2, line_no)?;

            let slot = parse_u8(tokens[1], line_no)?;

            Ok(Op::Store(slot))
        }

        "PRINT" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Print)
        }

        "HALT" => {
            expect_operands(&tokens, 1, line_no)?;
            Ok(Op::Halt)
        }

        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Line {}: unknown instruction '{}'",
                line_no,
                mnemonic
            ),
        )),
    }
}

fn expect_operands(
    tokens: &[&str],
    expected: usize,
    line_no: usize,
) -> io::Result<()> {

    if tokens.len() != expected {

        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "Line {}: expected {} token(s), found {}",
                line_no,
                expected,
                tokens.len()
            ),
        ));
    }

    Ok(())
}

fn parse_i64(text: &str, line_no: usize) -> io::Result<i64> {

    text.parse::<i64>()
        .map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Line {}: invalid integer '{}'",
                    line_no,
                    text
                ),
            )
        })
}

fn parse_u8(text: &str, line_no: usize) -> io::Result<u8> {

    text.parse::<u8>()
        .map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Line {}: invalid slot '{}'",
                    line_no,
                    text
                ),
            )
        })
}

fn encode_program(program: &[Op]) -> Vec<u8> {
    let mut bytes = Vec::new();

    for op in program {
        bytes.extend(op.encode());
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_push() {
        let src = "PUSH 42\nHALT";

        let program = parse_program(src).unwrap();

        assert_eq!(program.len(), 2);
        assert_eq!(program[0], Op::Push(42));
        assert_eq!(program[1], Op::Halt);
    }

    #[test]
    fn parse_case_insensitive() {
        let src = "push 7\nprint\nhalt";

        let program = parse_program(src).unwrap();

        assert_eq!(program[0], Op::Push(7));
        assert_eq!(program[1], Op::Print);
        assert_eq!(program[2], Op::Halt);
    }

    #[test]
    fn parse_comments() {
        let src = "
            ; comment
            PUSH 10 ; inline comment
            PRINT
            HALT
        ";

        let program = parse_program(src).unwrap();

        assert_eq!(program.len(), 3);
    }

    #[test]
    fn parse_unknown_instruction() {
        let src = "HELLO";

        assert!(parse_program(src).is_err());
    }

    #[test]
    fn encode_program_test() {
        let program = vec![
            Op::Push(7),
            Op::Push(3),
            Op::Add,
            Op::Print,
            Op::Halt,
        ];

        let bytes = encode_program(&program);

        assert!(!bytes.is_empty());
    }

    #[test]
    fn parse_load_store() {
        let src = "
            LOAD 5
            STORE 10
            HALT
        ";

        let program = parse_program(src).unwrap();

        assert_eq!(program[0], Op::Load(5));
        assert_eq!(program[1], Op::Store(10));
    }

    #[test]
    fn invalid_slot() {
        let src = "LOAD 999";

        assert!(parse_program(src).is_err());
    }

    #[test]
    fn invalid_integer() {
        let src = "PUSH hello";

        assert!(parse_program(src).is_err());
    }

    #[test]
    fn missing_operand() {
        let src = "PUSH";

        assert!(parse_program(src).is_err());
    }

    #[test]
    fn extra_operand() {
        let src = "ADD 10";

        assert!(parse_program(src).is_err());
    }

    #[test]
    fn inline_comment() {
        let src = "PUSH 10 ; comment\nHALT";

        let program = parse_program(src).unwrap();

        assert_eq!(program.len(), 2);
        assert_eq!(program[0], Op::Push(10));
    }
    
    #[test]
    fn blank_lines() {
        let src = "\n\nPUSH 5\n\nHALT\n";

        let program = parse_program(src).unwrap();

        assert_eq!(program.len(), 2);
    }

    #[test]
    fn negative_push() {
        let src = "PUSH -42\nHALT";

        let program = parse_program(src).unwrap();

        assert_eq!(program[0], Op::Push(-42));
    }
}