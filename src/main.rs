#[macro_use]
mod log_macros;

mod asm;
mod parser;
mod vm;

extern crate clap;
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("uvm")
        .version("0.1.0")
        .author("Victhor Sartório <victhor@victhor.io>")
        .about("The Useless Virtual Machine, implemented in Rust")
        .arg(
            Arg::new("program_path")
                .help("Path to the UVM assembly program to be executed")
                .required(true),
        )
        .arg(
            Arg::new("batched_output")
                .short('b')
                .long("batched-output")
                .help("Prints the output of the program only at the end of execution")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let program_path = matches.get_one::<String>("program_path").unwrap();
    let batched_output = matches.get_flag("batched_output");

    run(program_path.clone(), batched_output);
}

fn run(input_path: String, batched_output: bool) {
    let code = parser::parse_file(input_path);
    if code.is_err() {
        let err = code.unwrap_err();
        println!("{}", err);
        std::process::exit(1);
    }
    let code = code.unwrap();

    asm::display_code(&code);

    let mut vm = vm::VM::new(code);
    if batched_output {
        vm = vm.capture_output();
    }

    let result = vm.run();
    if result.is_err() {
        println!("{}", result.unwrap_err());
        std::process::exit(1);
    }
    if batched_output {
        println!("{}", result.unwrap());
    }
}
