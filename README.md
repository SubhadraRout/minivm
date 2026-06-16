# MiniVM

A stack-based virtual machine written in **Rust** using only the Rust standard library (`std`). MiniVM consists of a single-pass assembler, a bytecode virtual machine, and a disassembler. Assembly programs are compiled into a custom bytecode format and executed by the virtual machine.

---

## Features

* Stack-based Virtual Machine
* Single-pass Assembler
* Bytecode Disassembler
* Custom Binary Bytecode Format
* Runtime Trap Handling
* Execution Trace Mode (`--trace`)
* Zero External Dependencies (`std` only)

---

## Project Structure

```text
minivm/
├── src/
│   ├── assembler.rs
│   ├── bytecode.rs
│   ├── disassembler.rs
│   ├── errors.rs
│   ├── isa.rs
│   ├── main.rs
│   └── vm.rs
│
├── test_programs/
│   ├── arith.tasm
│   ├── celsius.tasm
│   ├── digits.tasm
│   ├── horner.tasm
│   ├── stackplay.tasm
│   ├── stack_underflow.tasm
│   ├── divide_by_zero.tasm
│   ├── modulo_by_zero.tasm
│   └── no_halt.tasm
│
├── Cargo.toml
└── README.md
```

---

# Building

```bash
cargo build
```

---

# Usage

## Assemble

```bash
cargo run -- asm <file.tasm> -o <file.tbc>
```

Example

```bash
cargo run -- asm test_programs/arith.tasm -o arith.tbc
```

---

## Execute

```bash
cargo run -- run <file.tbc>
```

Example

```bash
cargo run -- run arith.tbc
```

---

## Execute with Trace

```bash
cargo run -- run --trace <file.tbc>
```

Example

```bash
cargo run -- run --trace arith.tbc
```

---

## Disassemble

```bash
cargo run -- dis <file.tbc> -o <file.tasm>
```

Example

```bash
cargo run -- dis arith.tbc -o output.tasm
```

---

# Example Output

Normal execution

```text
10
```

Trace execution

```text
ip=0x0000 Push(7) stack=[]
ip=0x0009 Push(3) stack=[7]
ip=0x0012 Add stack=[7, 3]
ip=0x0013 Push(9) stack=[10]
ip=0x001C Push(4) stack=[10, 9]
ip=0x0025 Sub stack=[10, 9, 4]
ip=0x0026 Mul stack=[10, 5]
ip=0x0027 Push(5) stack=[50]
ip=0x0030 Div stack=[50, 5]
ip=0x0031 Print stack=[10]
10
ip=0x0032 Halt stack=[]
```

---

# Instruction Set Architecture (ISA)

| Opcode | Mnemonic | Description         |
| -----: | -------- | ------------------- |
|   0x01 | PUSH     | Push 64-bit integer |
|   0x02 | POP      | Pop top value       |
|   0x03 | DUP      | Duplicate top value |
|   0x04 | SWAP     | Swap top two values |
|   0x10 | ADD      | Addition            |
|   0x11 | SUB      | Subtraction         |
|   0x12 | MUL      | Multiplication      |
|   0x13 | DIV      | Integer division    |
|   0x14 | MOD      | Modulo              |
|   0x15 | NEG      | Negation            |
|   0x40 | LOAD     | Load global slot    |
|   0x41 | STORE    | Store global slot   |
|   0x60 | PRINT    | Pop and print value |
|   0xFF | HALT     | Stop execution      |

---

# Bytecode File Format

```
+--------------------+
| Magic ("MVM\0")    |
+--------------------+
| Version (1 byte)   |
+--------------------+
| Code Length (u32)  |
+--------------------+
| Raw Bytecode       |
+--------------------+
```

Magic bytes

```
4D 56 4D 00
```

Version

```
01
```

---

# Runtime Traps

MiniVM detects the following runtime traps:

* Stack overflow
* Stack underflow
* Division by zero
* Modulo by zero
* Unknown opcode
* Truncated instruction
* Program ended without HALT

Example

```text
trap at ip=0x0013: division by zero
```

---

# Acceptance Programs

| Program        | Description                                                  | Expected Output |
| -------------- | ------------------------------------------------------------ | --------------- |
| arith.tasm     | Computes (7 + 3) × (9 − 4) ÷ 5                               | 10              |
| celsius.tasm   | Converts 100°C to Fahrenheit                                 | 212             |
| horner.tasm    | Evaluates 3x³ + 2x² + 5x + 7 at x = 11 using Horner's method | 4297            |
| stackplay.tasm | Computes 12² + 35² using DUP, SWAP and global storage        | 1369            |
| digits.tasm    | Prints the digits of 9274 using only DIV and MOD             | 9 2 7 4         |

---

# Infix → Stack Translation

| Infix Expression      | Stack Code                                                                                                                                                             |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (7 + 3) × (9 − 4) ÷ 5 | `PUSH 7 → PUSH 3 → ADD → PUSH 9 → PUSH 4 → SUB → MUL → PUSH 5 → DIV`                                                                                                   |
| 100 × 9 ÷ 5 + 32      | `PUSH 100 → PUSH 9 → MUL → PUSH 5 → DIV → PUSH 32 → ADD`                                                                                                               |
| 3x³ + 2x² + 5x + 7    | `STORE x → PUSH 3 → LOAD x → MUL → PUSH 2 → ADD → LOAD x → MUL → PUSH 5 → ADD → LOAD x → MUL → PUSH 7 → ADD` *(evaluated using Horner's method: ((3x + 2)x + 5)x + 7)* |
| a² + b²               | `STORE a → STORE b → LOAD a → DUP → SWAP → MUL → LOAD b → DUP → SWAP → MUL → ADD`                                                                                      |
| Digits of 9274        | `DIV` and `MOD` operations extract the thousands, hundreds, tens and ones digits without loops.                                                                        |

---

# Technologies Used

* Rust
* Rust Standard Library (`std`)
* Cargo
* Git

---

# Author

**Subhadra Rout**
