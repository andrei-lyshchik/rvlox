extern crate rvlox;

use std::env;
use std::process;

use rvlox::util::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    let running_mode = parse_args_for_running_mode(&args).unwrap_or_else(|err| {
        println!("Error: invalid arguments {}", err);
        process::exit(1);
    });

    match running_mode {
        RunningMode::Script(file_name) => run_file(file_name),
        RunningMode::Repl => run_repl(),
    }

}