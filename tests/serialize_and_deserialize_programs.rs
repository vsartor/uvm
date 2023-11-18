use uvm;

#[test]
fn serialize_and_deserialize_fibonacci() {
    let code = uvm::parser::parse_file("tests/recursive_fibonacci.uvm".to_string());
    if !code.is_ok() {
        println!("{}", code.unwrap_err());
        assert!(false);
        return;
    }
    let code = code.unwrap();

    let binary = uvm::serializer::serialize(&code);
    if !binary.is_ok() {
        println!("{}", binary.unwrap_err());
        assert!(false);
        return;
    }
    let binary = binary.unwrap();

    let deserialized = uvm::serializer::deserialize(binary);
    if !deserialized.is_ok() {
        println!("{}", deserialized.unwrap_err());
        assert!(false);
        return;
    }
    let deserialized = deserialized.unwrap();

    assert_eq!(code, deserialized);
}

#[test]
fn serialize_and_deserialize_conditional_jump_tests() {
    let code = uvm::parser::parse_file("tests/conditional_jump_tests.uvm".to_string());
    if !code.is_ok() {
        println!("{}", code.unwrap_err());
        assert!(false);
        return;
    }
    let code = code.unwrap();

    let binary = uvm::serializer::serialize(&code);
    if !binary.is_ok() {
        println!("{}", binary.unwrap_err());
        assert!(false);
        return;
    }
    let binary = binary.unwrap();

    let deserialized = uvm::serializer::deserialize(binary);
    if !deserialized.is_ok() {
        println!("{}", deserialized.unwrap_err());
        assert!(false);
        return;
    }
    let deserialized = deserialized.unwrap();

    assert_eq!(code, deserialized);
}
