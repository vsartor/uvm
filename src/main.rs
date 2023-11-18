use uvm::{asm, parser, serializer, vm};

extern crate clap;
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("uvm")
        .version("0.1.0")
        .author("Victhor Sartório <victhor@victhor.io>")
        .about("The Useless Virtual Machine, implemented in Rust")
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .about("Runs a UVM program from either source or bytecode")
                .arg(
                    Arg::new("program_path")
                        .required(true)
                        .help("Path to the program to be run"),
                )
                .arg(
                    Arg::new("binary")
                        .short('b')
                        .long("binary")
                        .action(ArgAction::SetTrue)
                        .help("Treat the program as a binary bytecode file instead of source code"),
                )
                .arg(
                    Arg::new("batched_output")
                        .short('o')
                        .long("batched-output")
                        .action(ArgAction::SetTrue)
                        .help("Capture the output of the program and print it all at once"),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue),
                )
                .arg(Arg::new("debug").short('d').long("debug").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("asm")
                .about("Assembles a UVM program from source code")
                .arg(
                    Arg::new("input_path")
                        .required(true)
                        .help("Path to the program to be assembled"),
                )
                .arg(Arg::new("output_path").required(true).help("Path to the output file")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            // required, so it's safe to unwrap
            let input_path = run_matches.get_one::<String>("program_path").unwrap().clone();
            let is_binary = run_matches.get_flag("binary");
            let is_batched_output = run_matches.get_flag("batched_output");
            let is_debug = run_matches.get_flag("debug");
            let is_verbose = run_matches.get_flag("verbose");

            if is_binary {
                let code = serializer::disassemble(input_path);
                if code.is_err() {
                    let err = code.unwrap_err();
                    println!("{}", err);
                    std::process::exit(1);
                }
                let code = code.unwrap();
                run(code, is_batched_output, is_debug, is_verbose);
            } else {
                let code = parser::parse_file(input_path);
                if code.is_err() {
                    let err = code.unwrap_err();
                    println!("{}", err);
                    std::process::exit(1);
                }
                let code = code.unwrap();
                run(code, is_batched_output, is_debug, is_verbose);
            }
        }
        Some(("asm", asm_matches)) => {
            // required, so it's safe to unwrap
            let input_path = asm_matches.get_one::<String>("input_path").unwrap().clone();
            let output_path = asm_matches.get_one::<String>("output_path").unwrap().clone();

            let asm_result = serializer::assemble(input_path, output_path);
            if asm_result.is_err() {
                println!("{}", asm_result.unwrap_err());
                std::process::exit(1);
            }
        }
        _ => unreachable!(),
    }
}

fn run(code: Vec<asm::Code>, is_batched_output: bool, is_debug: bool, is_verbose: bool) {
    if is_verbose {
        asm::display_code(&code);
    }

    let mut vm = vm::VM::new(code);
    if is_batched_output {
        vm = vm.capture_output();
    }

    let result = if !is_debug { vm.run() } else { vm.debugger() };
    if result.is_err() {
        println!("{}", result.unwrap_err());
        std::process::exit(1);
    }
    if is_batched_output {
        println!("{}", result.unwrap());
    }
}
