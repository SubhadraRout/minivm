mod assembler;
mod bytecode;
mod disassembler;
mod errors;
mod isa;
mod vm;

use std::env;
use std::process;

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  minivm asm <input.tasm> -o <output.tbc>");
    eprintln!("  minivm run <input.tbc>");
    eprintln!("  minivm dis <input.tbc> -o <output.tasm>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "asm" => {
            if args.len() != 5 || args[3] != "-o" {
                print_usage();
                process::exit(1);
            }

            if let Err(e) = assembler::assemble_file(&args[2], &args[4]) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }

        "run" => {
            if args.len() != 3 {
                print_usage();
                process::exit(1);
            }

            if let Err(e) = vm::Vm::run_file(&args[2]) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }

        "dis" => {
            if args.len() != 5 || args[3] != "-o" {
                print_usage();
                process::exit(1);
            }

            if let Err(e) = disassembler::disassemble_file(&args[2], &args[4]) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }

        _ => {
            print_usage();
            process::exit(1);
        }
    }
}