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
    assert_eq!(vm.get_registers()[1..14], [22, 10, 50, 5, 3, 11, 9, -11, -30, 8, -8, 4, 9]);
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

#[test]
fn test_cmp() {
    let code = uvm::parser::parse_file("tests/cmp_test.uvm".to_string());
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
    assert_eq!(vm.get_cmp(), 1);
}

#[test]
fn test_conditional_jumps() {
    let code = uvm::parser::parse_file("tests/conditional_jump_tests.uvm".to_string());
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
    assert_eq!(vm.get_registers()[7], 1);
}

#[test]
fn test_basic_stack() {
    let code = uvm::parser::parse_file("tests/basic_stack.uvm".to_string());
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
    assert_eq!(vm.get_registers()[..3], [0, 20, 10]);
}

#[test]
fn test_rf_stack_ops() {
    let code = uvm::parser::parse_file("tests/rf_push_pop.uvm".to_string());
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
    assert_eq!(vm.get_registers()[..10], [10, 11, 12, 13, 14, 15, 16, 17, 0, 0]);
}

#[test]
fn test_fibonacci_recursion() {
    let code = uvm::parser::parse_file("tests/recursive_fibonacci.uvm".to_string());
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
    assert_eq!(vm.get_registers()[0], 6765);
}

#[test]
fn test_basic_float_arithmetic() {
    let code = uvm::parser::parse_file("tests/basic_float_arithmetic.uvm".to_string());
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
    assert_eq!(vm.get_registers_as_floats()[..5], [3.14, 10.99, 4.396, -87.92, 88.02]);
    assert_eq!(vm.get_registers()[5..9], [89, 88, 1405, 25796]);
    assert_eq!(vm.get_registers_as_floats()[9..13], [11.2, -11.2, 7.0, 0.5]);
}
