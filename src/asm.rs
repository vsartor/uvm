#[derive(Copy, Clone)]
pub enum OpCode {
    HALT,
    SET,
    ADD,
    DBGREG,
    DBGREGS,
}

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            OpCode::HALT => write!(f, "HALT"),
            OpCode::SET => write!(f, "SET"),
            OpCode::ADD => write!(f, "ADD"),
            OpCode::DBGREG => write!(f, "DBGREG"),
            OpCode::DBGREGS => write!(f, "DBGREGS"),
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
            "DBGREG" => Ok(OpCode::DBGREG),
            "DBGREGS" => Ok(OpCode::DBGREGS),
            _ => Err(err!("Failed to parse opcode: {}", s)),
        }
    }
}

pub enum Code {
    Op(OpCode),
    Reg(u8),
    Int(i64),
}

impl std::fmt::Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Code::Op(op) => write!(f, "Code({:?})", op),
            Code::Reg(reg) => write!(f, "Reg({})", reg),
            Code::Int(val) => write!(f, "Int({})", val),
        }
    }
}
