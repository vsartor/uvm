use uvm;

#[test]
fn test_basic_arithmetics() {
    let code = uvm::parser::parse_file("tests/basic_arithmetic.uvm".to_string());
    if !code.is_ok() {
        println!("{}", code.unwrap_err());
        assert!(false);
        return;
    }
    let mut vm = uvm::vm::VM::new(code.unwrap());
    let result = vm.run();
    if !result.is_ok() {
        println!("{}", result.unwrap_err());
        assert!(false);
        return;
    }
    assert_eq!(vm.get_registers()[1..6], [22, 10, 50, 5, 3]);
}

#[test]
fn test_basic_loop() {
    let code = uvm::parser::parse_file("tests/basic_loop.uvm".to_string());
    if !code.is_ok() {
        println!("{}", code.unwrap_err());
        assert!(false);
        return;
    }
    let mut vm = uvm::vm::VM::new(code.unwrap());
    let result = vm.run();
    if !result.is_ok() {
        println!("{}", result.unwrap_err());
        assert!(false);
        return;
    }
    assert_eq!(vm.get_registers()[0..2], [0, 1275]);
}
