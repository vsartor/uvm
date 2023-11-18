#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    HALT,
    SET,
    PUSH,
    POP,
    PUSHRF,
    POPRF,
    ADD,
    ADDL,
    SUB,
    SUBLA,
    SUBLB,
    MUL,
    MULL,
    DIV,
    DIVLA,
    DIVLB,
    MOD,
    INC,
    DEC,
    CMP,
    CMPL,
    JMP,
    JEQ,
    JLT,
    JLE,
    JGT,
    JGE,
    JNE,
    CALL,
    RET,
    DBGREG,
    DBGREGS,
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match f.align() {
            None => match self {
                OpCode::HALT => write!(f, "HALT"),
                OpCode::SET => write!(f, "SET"),
                OpCode::PUSH => write!(f, "PUSH"),
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
            "PUSH" => Ok(OpCode::PUSH),
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
}

pub const OP_ARG_TYPES: [OpArgT; 32] = [
    OpArgT::Nil,    // HALT
    OpArgT::IntReg, // SET
    OpArgT::Reg,    // PUSH
    OpArgT::Reg,    // POP
    OpArgT::Int,    // PUSHRF
    OpArgT::Int,    // POPRF
    OpArgT::RegReg, // ADD
    OpArgT::IntReg, // ADDL
    OpArgT::RegReg, // SUB
    OpArgT::IntReg, // SUBLA
    OpArgT::IntReg, // SUBLB
    OpArgT::RegReg, // MUL
    OpArgT::IntReg, // MULL
    OpArgT::RegReg, // DIV
    OpArgT::IntReg, // DIVLA
    OpArgT::IntReg, // DIVLB
    OpArgT::RegReg, // MOD
    OpArgT::Reg,    // INC
    OpArgT::Reg,    // DEC
    OpArgT::RegReg, // CMP
    OpArgT::IntReg, // CMPL
    OpArgT::Addr,   // JMP
    OpArgT::Addr,   // JEQ
    OpArgT::Addr,   // JLT
    OpArgT::Addr,   // JLE
    OpArgT::Addr,   // JGT
    OpArgT::Addr,   // JGE
    OpArgT::Addr,   // JNE
    OpArgT::Addr,   // CALL
    OpArgT::Nil,    // RET
    OpArgT::Reg,    // DBGREG
    OpArgT::Nil,    // DBGREGS
];

#[derive(Debug, PartialEq)]
pub enum Code {
    Op(OpCode),
    Reg(u8),
    Int(i64),
    Addr(usize),
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match f.align() {
            None => match self {
                Code::Op(op) => write!(f, "\x1b[1m{}\x1b[0m", op),
                Code::Reg(reg) => write!(f, "r{}", reg),
                Code::Int(val) => write!(f, "{}i", val),
                Code::Addr(addr) => write!(f, "addr({})", addr),
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
        }
    }

    println!("└ END\n"); // note the trailing newline
}
