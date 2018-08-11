use std::fs::File;
use std::io::Read;
use std::process;
use std::io::{self};

use vm::interpret_source;
use vm::InterpretResult;

pub fn read_file_to_string(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    Ok(source)
}

#[derive(Debug)]
pub enum RunningMode {
    Script(String),
    Repl,
}

pub fn parse_args_for_running_mode(args: &Vec<String>) -> Result<RunningMode, &'static str> {
    if args.len() > 2 {
        return Err("Usage: rlox [script]")
    }

    if args.len() > 1 {
        let script_file_name = args[1].clone();
        Ok(RunningMode::Script(script_file_name))
    } else {
        Ok(RunningMode::Repl)
    }
}

pub fn run_file(file_name: String) {
    let source = read_file_to_string(&file_name).unwrap_or_else(|err| {
        println!("Unable to read script file: {}", err);
        process::exit(2);
    });

    match interpret_source(&source) {
        InterpretResult::Ok => process::exit(0),
        InterpretResult::RuntimeError => process::exit(1),
        InterpretResult::CompileError => process::exit(2)
    }
}

pub fn run_repl() {
    println!("=== Rvlox repl ===");
}