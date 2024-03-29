mod chunk;
mod value;
mod vm;
mod opcodes;
mod tests;
mod token;
mod util;
mod lexer;
mod compiler;
mod parser;
mod precedence;
mod parse_rule;
mod macros;

use std::{ io::{ self, stdout, BufRead, Write }, net::TcpListener, process };

use util::print::print_error;
use vm::VM;

use crate::vm::InterpretResult;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            print_error("Usage: viskum [path]");
            std::process::exit(64);
        }
    }

    let mut vm = VM::new();

    vm.free_vm()
}

fn run(source: &str, vm: &mut VM) {
    match vm.interpret(source) {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => {}
        _ => {}
    }
}

fn run_file(path: &String) {
    let vm = &mut VM::new();
    match std::fs::read_to_string(path) {
        Ok(str) => run(str.as_str(), vm),
        Err(e) => {
            print_error(format!("There was an error while reading file: {}", e).as_str());
            process::exit(64);
        }
    }
}

fn run_prompt() {
    let vm = &mut VM::new();
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            run(line.as_str(), vm);
        } else {
            break;
        }
        print!("> ");
        let _ = stdout().flush();
    }
}
