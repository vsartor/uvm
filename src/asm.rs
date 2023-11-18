use std::collections::HashMap;

use num_enum::TryFromPrimitive;

#[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    HALT,    // Stops execution
    SET,     // x rb: Sets `rb` to `x`
    SETF,    // x rb: Sets `rb` to `x` as a floating point value
    MOV,     // ra rb: Sets `rb` to `ra`
    PUSH,    // rb: Pushes the value of `rb` to the stack
    PUSHL,   // x: Pushes `x` to the stack
    POP,     // rb: Pops the top of the stack to `rb`
    PUSHRF,  // x: Saves the value of the first `n` registers to the stack
    POPRF,   // x: Loads the value of the first `n` registers from the stack
    ADD,     // ra rb: Adds `ra` and `rb` and stores the result in `rb`
    ADDL,    // x rb: Adds `x` and `rb` and stores the result in `rb`
    SUB,     // ra rb: Subtracts `ra` from `rb` and stores the result in `rb`
    SUBLA,   // x rb: Subtracts `x` from `rb` and stores the result in `rb`
    SUBLB,   // x rb: Subtracts `rb` from `x` and stores the result in `rb`
    MUL,     // ra rb: Multiplies `ra` and `rb` and stores the result in `rb`
    MULL,    // x rb: Multiplies `x` and `rb` and stores the result in `rb`
    DIV,     // ra rb: Divides `rb` by `ra` and stores the result in `rb`
    DIVLA,   // x rb: Divides `rb` by `x` and stores the result in `rb`
    DIVLB,   // x rb: Divides `x` by `rb` and stores the result in `rb`
    MOD,     // ra rb: Stores the remainder of `rb` divided by `ra` in `rb`
    INC,     // rb: Increments `rb` by 1
    DEC,     // rb: Decrements `rb` by 1
    ADDF,    // ra rb: Floating point adds `ra` and `rb` and stores the result in `rb`
    SUBF,    // ra rb: Floating point subtracts `ra` from `rb` and stores the result in `rb`
    MULF,    // ra rb: Floating point multiplies `ra` and `rb` and stores the result in `rb`
    DIVF,    // ra rb: Floating point divides `rb` by `ra` and stores the result in `rb`
    CEIL,    // rb: Rounds `rb` up to the nearest integer
    FLOR,    // rb: Rounds `rb` down to the nearest integer
    CMP,     // ra rb: Compares `rb` and `ra` and stores the result in `cmp` (e.g. GT if `rb` > `ra`)
    CMPL,    // x rb: Compares `rb` and `x` and stores the result in `cmp` (e.g. GT if `rb` > `x`)
    JMP,     // addr: Jumps to `addr`
    JEQ,     // addr: Jumps to `addr` if `cmp` has EQ
    JLT,     // addr: Jumps to `addr` if `cmp` has LT
    JLE,     // addr: Jumps to `addr` if `cmp` has LE
    JGT,     // addr: Jumps to `addr` if `cmp` has GT
    JGE,     // addr: Jumps to `addr` if `cmp` has GE
    JNE,     // addr: Jumps to `addr` if `cmp` has NE
    CALL,    // addr: Calls the function at `addr` saving the current address in the call stack
    RET,     // Returns from a function (pops the call stack and jumps to the saved address)
    DBGREG,  // rb: Prints the value of `rb` to stdout for debugging
    DBGREGF, // rb: Prints the value of `rb` as a floating point value to stdout for debugging
    DBGREGS, // Prints the values of all registers to stdout for debugging
}

impl OpCode {
    pub fn to_le_bytes(&self) -> [u8; 1] {
        (*self as u8).to_le_bytes()
    }

    pub fn from_le_bytes(bytes: [u8; 1]) -> Option<OpCode> {
        let byte = u8::from_le_bytes(bytes);
        let op = OpCode::try_from_primitive(byte);
        if op.is_err() {
            return None;
        }
        Some(op.unwrap())
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match f.align() {
            None => match self {
                OpCode::HALT => write!(f, "HALT"),
                OpCode::SET => write!(f, "SET"),
                OpCode::SETF => write!(f, "SETF"),
                OpCode::MOV => write!(f, "MOV"),
                OpCode::PUSH => write!(f, "PUSH"),
                OpCode::PUSHL => write!(f, "PUSHL"),
                OpCode::POP => write!(f, "POP"),
                OpCode::PUSHRF => write!(f, "PUSHRF"),
                OpCode::POPRF => write!(f, "POPRF"),
                OpCode::ADD => write!(f, "ADD"),
                OpCode::ADDL => write!(f, "ADDL"),
                OpCode::SUB => write!(f, "SUB"),
                OpCode::SUBLA => write!(f, "SUBLA"),
                OpCode::SUBLB => write!(f, "SUBLB"),
                OpCode::MUL => write!(f, "MUL"),
                OpCode::MULL => write!(f, "MULL"),
                OpCode::DIV => write!(f, "DIV"),
                OpCode::DIVLA => write!(f, "DIVLA"),
                OpCode::DIVLB => write!(f, "DIVLB"),
                OpCode::MOD => write!(f, "MOD"),
                OpCode::INC => write!(f, "INC"),
                OpCode::DEC => write!(f, "DEC"),
                OpCode::ADDF => write!(f, "ADDF"),
                OpCode::SUBF => write!(f, "SUBF"),
                OpCode::MULF => write!(f, "MULF"),
                OpCode::DIVF => write!(f, "DIVF"),
                OpCode::CEIL => write!(f, "CEIL"),
                OpCode::FLOR => write!(f, "FLOR"),
                OpCode::CMP => write!(f, "CMP"),
                OpCode::CMPL => write!(f, "CMPL"),
                OpCode::JMP => write!(f, "JMP"),
                OpCode::JEQ => write!(f, "JEQ"),
                OpCode::JLT => write!(f, "JLT"),
                OpCode::JLE => write!(f, "JLE"),
                OpCode::JGT => write!(f, "JGT"),
                OpCode::JGE => write!(f, "JGE"),
                OpCode::JNE => write!(f, "JNE"),
                OpCode::CALL => write!(f, "CALL"),
                OpCode::RET => write!(f, "RET"),
                OpCode::DBGREG => write!(f, "DBGREG"),
                OpCode::DBGREGF => write!(f, "DBGREGF"),
                OpCode::DBGREGS => write!(f, "DBGREGS"),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

impl std::str::FromStr for OpCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HALT" => Ok(OpCode::HALT),
            "SET" => Ok(OpCode::SET),
            "SETF" => Ok(OpCode::SETF),
            "MOV" => Ok(OpCode::MOV),
            "PUSH" => Ok(OpCode::PUSH),
            "PUSHL" => Ok(OpCode::PUSHL),
            "POP" => Ok(OpCode::POP),
            "PUSHRF" => Ok(OpCode::PUSHRF),
            "POPRF" => Ok(OpCode::POPRF),
            "ADD" => Ok(OpCode::ADD),
            "ADDL" => Ok(OpCode::ADDL),
            "SUB" => Ok(OpCode::SUB),
            "SUBLA" => Ok(OpCode::SUBLA),
            "SUBLB" => Ok(OpCode::SUBLB),
            "MUL" => Ok(OpCode::MUL),
            "MULL" => Ok(OpCode::MULL),
            "DIV" => Ok(OpCode::DIV),
            "DIVLA" => Ok(OpCode::DIVLA),
            "DIVLB" => Ok(OpCode::DIVLB),
            "MOD" => Ok(OpCode::MOD),
            "INC" => Ok(OpCode::INC),
            "DEC" => Ok(OpCode::DEC),
            "ADDF" => Ok(OpCode::ADDF),
            "SUBF" => Ok(OpCode::SUBF),
            "MULF" => Ok(OpCode::MULF),
            "DIVF" => Ok(OpCode::DIVF),
            "CEIL" => Ok(OpCode::CEIL),
            "FLOR" => Ok(OpCode::FLOR),
            "CMP" => Ok(OpCode::CMP),
            "CMPL" => Ok(OpCode::CMPL),
            "JMP" => Ok(OpCode::JMP),
            "JEQ" => Ok(OpCode::JEQ),
            "JLT" => Ok(OpCode::JLT),
            "JLE" => Ok(OpCode::JLE),
            "JGT" => Ok(OpCode::JGT),
            "JGE" => Ok(OpCode::JGE),
            "JNE" => Ok(OpCode::JNE),
            "CALL" => Ok(OpCode::CALL),
            "RET" => Ok(OpCode::RET),
            "DBGREG" => Ok(OpCode::DBGREG),
            "DBGREGF" => Ok(OpCode::DBGREGF),
            "DBGREGS" => Ok(OpCode::DBGREGS),
            _ => Err(err!("Failed to parse opcode: {}", s)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum OpArgT {
    Nil,
    Reg,
    IntReg,
    RegReg,
    Addr,
    Int,
    RealReg,
}

pub const OP_ARG_TYPES: [OpArgT; 42] = [
    OpArgT::Nil,     // HALT
    OpArgT::IntReg,  // SET
    OpArgT::RealReg, // SETF
    OpArgT::RegReg,  // MOV
    OpArgT::Reg,     // PUSH
    OpArgT::Int,     // PUSHL
    OpArgT::Reg,     // POP
    OpArgT::Int,     // PUSHRF
    OpArgT::Int,     // POPRF
    OpArgT::RegReg,  // ADD
    OpArgT::IntReg,  // ADDL
    OpArgT::RegReg,  // SUB
    OpArgT::IntReg,  // SUBLA
    OpArgT::IntReg,  // SUBLB
    OpArgT::RegReg,  // MUL
    OpArgT::IntReg,  // MULL
    OpArgT::RegReg,  // DIV
    OpArgT::IntReg,  // DIVLA
    OpArgT::IntReg,  // DIVLB
    OpArgT::RegReg,  // MOD
    OpArgT::Reg,     // INC
    OpArgT::Reg,     // DEC
    OpArgT::RegReg,  // ADDF
    OpArgT::RegReg,  // SUBF
    OpArgT::RegReg,  // MULF
    OpArgT::RegReg,  // DIVF
    OpArgT::Reg,     // CEIL
    OpArgT::Reg,     // FLOR
    OpArgT::RegReg,  // CMP
    OpArgT::IntReg,  // CMPL
    OpArgT::Addr,    // JMP
    OpArgT::Addr,    // JEQ
    OpArgT::Addr,    // JLT
    OpArgT::Addr,    // JLE
    OpArgT::Addr,    // JGT
    OpArgT::Addr,    // JGE
    OpArgT::Addr,    // JNE
    OpArgT::Addr,    // CALL
    OpArgT::Nil,     // RET
    OpArgT::Reg,     // DBGREG
    OpArgT::Reg,     // DBGREGF
    OpArgT::Nil,     // DBGREGS
];

#[derive(Debug, PartialEq)]
pub enum Code {
    Op(OpCode),
    Reg(u8),
    Int(i64),
    Addr(usize),
    Real(f64),
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match f.align() {
            None => match self {
                Code::Op(op) => write!(f, "\x1b[1m{}\x1b[0m", op),
                Code::Reg(reg) => write!(f, "r{}", reg),
                Code::Int(val) => write!(f, "{}i", val),
                Code::Addr(addr) => write!(f, "addr({})", addr),
                Code::Real(val) => write!(f, "{}f", val),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

pub fn display_code(code: &Vec<Code>) {
    // We assume that the code is valid for this function and make
    // gratuitous use of unwrap().

    println!("{}", info!("Displaying loaded code below:"));
    println!("┌ START");

    let mut idx = 0;
    while idx < code.len() {
        let op = match code[idx] {
            Code::Op(op) => op,
            _ => {
                panic!("Expected an opcode, but got {}", code[idx])
            }
        };
        let arg_t = OP_ARG_TYPES[op as usize];
        match arg_t {
            OpArgT::Nil => {
                println!("│ {:04} {}", idx, code[idx]);
                idx += 1;
            }
            OpArgT::Reg => {
                println!("│ {:04} {} {}", idx, code[idx], code[idx + 1]);
                idx += 2;
            }
            OpArgT::IntReg => {
                println!("│ {:04} {} {} {}", idx, code[idx], code[idx + 1], code[idx + 2]);
                idx += 3;
            }
            OpArgT::RegReg => {
                println!("│ {:04} {} {} {}", idx, code[idx], code[idx + 1], code[idx + 2]);
                idx += 3;
            }
            OpArgT::Addr => {
                println!("│ {:04} {} {}", idx, code[idx], code[idx + 1]);
                idx += 2;
            }
            OpArgT::Int => {
                println!("│ {:04} {} {}", idx, code[idx], code[idx + 1]);
                idx += 2;
            }
            OpArgT::RealReg => {
                println!("│ {:04} {} {} {}", idx, code[idx], code[idx + 1], code[idx + 2]);
                idx += 3;
            }
        }
    }

    println!("└ END\n"); // note the trailing newline
}

pub fn displayable_code(code: &Vec<Code>) -> (Vec<String>, HashMap<usize, usize>, HashMap<usize, usize>) {
    // We return:
    // - A vector of strings representing each instruction.
    // - A hashmap mapping the address of each instruction to its index in the vector.
    // - A hashmap mapping the index of each instruction to its address (the inverse hashmap).

    // So for example if code is:
    // code = [SET, 0, r1, ADD, 1, 2]
    // We will return a vector of
    // ["SET 0 r1", "ADD 1 2"]
    // a hashmap of
    // {0: 0, 3: 1}
    // and a hashmap of
    // {0: 0, 1: 3}

    let mut addr2idx = HashMap::new();
    let mut displayable_code = Vec::new();

    let mut idx = 0;
    while idx < code.len() {
        let op = match code[idx] {
            Code::Op(op) => op,
            _ => {
                panic!("Expected an opcode, but got {}", code[idx])
            }
        };
        let arg_t = OP_ARG_TYPES[op as usize];
        match arg_t {
            OpArgT::Nil => {
                displayable_code.push(format!("{}", code[idx]));
                addr2idx.insert(idx, displayable_code.len() - 1);
                idx += 1;
            }
            OpArgT::Reg => {
                displayable_code.push(format!("{} {}", code[idx], code[idx + 1]));
                addr2idx.insert(idx, displayable_code.len() - 1);
                idx += 2;
            }
            OpArgT::IntReg => {
                displayable_code.push(format!("{} {} {}", code[idx], code[idx + 1], code[idx + 2]));
                addr2idx.insert(idx, displayable_code.len() - 1);
                idx += 3;
            }
            OpArgT::RegReg => {
                displayable_code.push(format!("{} {} {}", code[idx], code[idx + 1], code[idx + 2]));
                addr2idx.insert(idx, displayable_code.len() - 1);
                idx += 3;
            }
            OpArgT::Addr => {
                displayable_code.push(format!("{} {}", code[idx], code[idx + 1]));
                addr2idx.insert(idx, displayable_code.len() - 1);
                idx += 2;
            }
            OpArgT::Int => {
                displayable_code.push(format!("{} {}", code[idx], code[idx + 1]));
                addr2idx.insert(idx, displayable_code.len() - 1);
                idx += 2;
            }
            OpArgT::RealReg => {
                displayable_code.push(format!("{} {} {}", code[idx], code[idx + 1], code[idx + 2]));
                addr2idx.insert(idx, displayable_code.len() - 1);
                idx += 3;
            }
        }
    }

    let idx2addr = addr2idx.iter().map(|(k, v)| (*v, *k)).collect();

    (displayable_code, addr2idx, idx2addr)
}
