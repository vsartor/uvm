#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    HALT,
    SET,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    CMPL,
    JMP,
    JEQ,
    DBGREG,
    DBGREGS,
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match f.align() {
            None => match self {
                OpCode::HALT => write!(f, "HALT"),
                OpCode::SET => write!(f, "SET"),
                OpCode::ADD => write!(f, "ADD"),
                OpCode::SUB => write!(f, "SUB"),
                OpCode::MUL => write!(f, "MUL"),
                OpCode::DIV => write!(f, "DIV"),
                OpCode::MOD => write!(f, "MOD"),
                OpCode::CMPL => write!(f, "CMPL"),
                OpCode::JMP => write!(f, "JMP"),
                OpCode::JEQ => write!(f, "JEQ"),
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
            "ADD" => Ok(OpCode::ADD),
            "SUB" => Ok(OpCode::SUB),
            "MUL" => Ok(OpCode::MUL),
            "DIV" => Ok(OpCode::DIV),
            "MOD" => Ok(OpCode::MOD),
            "CMPL" => Ok(OpCode::CMPL),
            "JMP" => Ok(OpCode::JMP),
            "JEQ" => Ok(OpCode::JEQ),
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
}

pub const OP_ARG_TYPES: [OpArgT; 12] = [
    OpArgT::Nil,    // HALT
    OpArgT::IntReg, // SET
    OpArgT::RegReg, // ADD
    OpArgT::RegReg, // SUB
    OpArgT::RegReg, // MUL
    OpArgT::RegReg, // DIV
    OpArgT::RegReg, // MOD
    OpArgT::IntReg, // CMPL
    OpArgT::Addr,   // JMP
    OpArgT::Addr,   // JEQ
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
        }
    }

    println!("└ END\n"); // note the trailing newline
}
