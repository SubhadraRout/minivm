use std::convert::TryInto;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Push(i64),
    Pop,
    Dup,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Load(u8),
    Store(u8),
    Print,
    Halt,
}


impl Op {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Op::Push(value) => {
                let mut bytes = vec![0x01];
                bytes.extend(value.to_le_bytes());
                bytes
            }
            Op::Pop => vec![0x02],
            Op::Dup => vec![0x03],
            Op::Swap => vec![0x04],
            Op::Add => vec![0x10],
            Op::Sub => vec![0x11],
            Op::Mul => vec![0x12],
            Op::Div => vec![0x13],
            Op::Mod => vec![0x14],
            Op::Neg => vec![0x15],
            Op::Load(slot) => vec![0x40, *slot],
            Op::Store(slot) => vec![0x41, *slot],
            Op::Print => vec![0x60],
            Op::Halt => vec![0xFF],

        }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Op, usize), String>{
        if bytes.is_empty(){
            return Err("No bytes to decode".to_string());
        }
        match bytes[0]{
            0x01 => {
                if bytes.len() < 9 {
                    return Err("No bytes to decode".to_string());
                }
                let value = i64::from_le_bytes(bytes[1..9].try_into().unwrap());
                Ok((Op::Push(value), 9))
            }
            0x02 => Ok((Op::Pop, 1)),
            0x03 => Ok((Op::Dup, 1)),
            0x04 => Ok((Op::Swap, 1)),
            0x10 => Ok((Op::Add, 1)),
            0x11 => Ok((Op::Sub, 1)),
            0x12 => Ok((Op::Mul, 1)),
            0x13 => Ok((Op::Div, 1)),
            0x14 => Ok((Op::Mod, 1)),
            0x15 => Ok((Op::Neg, 1)),
            0x40 => {
                if bytes.len() < 2 {
                    return Err("Truncated LOAD instruction".to_string());
                }

                Ok((Op::Load(bytes[1]), 2))
            }
            0x41 => {
                if bytes.len() < 2 {
                    return Err("Truncated STORE instruction".to_string());
                }

                Ok((Op::Store(bytes[1]), 2))
            }
            0x60 => Ok((Op::Print, 1)),
            0xFF => Ok((Op::Halt, 1)),

            opcode => Err(format!("Unknown opcode: 0x{:02X}", opcode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_encode_decode() {
        let op = Op::Add;

        let bytes = op.encode();

        let (decoded, consumed) = Op::decode(&bytes).unwrap();

        assert_eq!(decoded, op);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn test_push_encode_decode() {
        let op = Op::Push(42);

        let bytes = op.encode();

        let (decoded, consumed) = Op::decode(&bytes).unwrap();

        assert_eq!(decoded, op);
        assert_eq!(consumed, 9);
    }

    #[test]
    fn test_load_encode_decode() {
        let op = Op::Load(7);

        let bytes = op.encode();

        let (decoded, consumed) = Op::decode(&bytes).unwrap();

        assert_eq!(decoded, op);
        assert_eq!(consumed, 2);
    }
}