use std::fs;
use std::io;

pub const MAGIC: [u8; 4] = [0x4D, 0x56, 0x4D, 0x00];
pub const VERSION: u8 = 0x01;

/// Write bytecode to a .tbc file
pub fn write_bytecode(path: &str, code: &[u8]) -> io::Result<()> {
    let mut bytes = Vec::new();

    // Magic bytes
    bytes.extend(MAGIC);

    // Version
    bytes.push(VERSION);

    // Code length (u32 little-endian)
    let len = code.len() as u32;
    bytes.extend(len.to_le_bytes());

    // Raw bytecode
    bytes.extend(code);

    fs::write(path, bytes)
}

/// Read and validate a .tbc file
pub fn read_bytecode(path: &str) -> io::Result<Vec<u8>> {
    let bytes = fs::read(path)?;

    // Header = 4 magic + 1 version + 4 length
    if bytes.len() < 9 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File too short",
        ));
    }

    // Validate magic
    if bytes[0..4] != MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid magic bytes",
        ));
    }

    // Validate version
    if bytes[4] != VERSION {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unsupported bytecode version",
        ));
    }

    // Read code length
    let length = u32::from_le_bytes([
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
    ]) as usize;

    // Validate file size
    if bytes.len() != 9 + length {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Incorrect code length",
        ));
    }

    Ok(bytes[9..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_then_read() {
        let code = vec![
            0x01,
            7,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0x60,
            0xFF,
        ];

        let path = "test_program.tbc";

        write_bytecode(path, &code).unwrap();

        let loaded = read_bytecode(path).unwrap();

        assert_eq!(code, loaded);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn invalid_magic() {
        let mut bytes = Vec::new();

        bytes.extend([1, 2, 3, 4]);
        bytes.push(VERSION);
        bytes.extend((0u32).to_le_bytes());

        fs::write("bad.tbc", bytes).unwrap();

        assert!(read_bytecode("bad.tbc").is_err());

        fs::remove_file("bad.tbc").unwrap();
    }
}