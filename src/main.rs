#[macro_use]
mod log_macros;
mod asm;
mod parser;
mod vm;

fn main() {
    // get command line arguments, but skip the first one (the program name)
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Usage: ./uvm <program> [flags]");
        std::process::exit(1);
    }

    let program = std::fs::read_to_string(&args[0]);
    if program.is_err() {
        println!("Failed to read file: {}", program.unwrap_err());
        // exit with a non-zero exit code
        std::process::exit(1);
    }
    let program = program.unwrap();

    let code = parser::parse_asm(program.to_string());
    if code.is_err() {
        let err = code.unwrap_err();
        println!("{}", err);
        std::process::exit(1);
    }
    let code = code.unwrap();

    asm::display_code(&code);

    let mut vm = vm::VM::new(code);

    let result = vm.run();
    if result.is_err() {
        println!("{}", result.unwrap_err());
        std::process::exit(1);
    }
}
