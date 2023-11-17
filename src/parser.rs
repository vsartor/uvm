use std::str::FromStr;

use crate::asm::{Code, OpArgT, OpCode, OP_ARG_TYPES};

struct Ctxt {
    filename: String,
    line: i64,
}

impl Ctxt {
    fn new(filename: String) -> Self {
        Self { filename, line: -1 }
    }

    fn inc(&mut self) {
        self.line += 1;
    }
}

pub fn parse_file(input_path: String) -> Result<Vec<Code>, String> {
    let program = std::fs::read_to_string(&input_path);
    if program.is_err() {
        return Err(format!("std::fs::read_to_string({}) => {}", &input_path, program.unwrap_err()));
    }
    let program = program.unwrap();

    parse_string(&program, Ctxt::new(input_path))
}

fn parse_string(raw_code: &String, mut ctxt: Ctxt) -> Result<Vec<Code>, String> {
    let mut code = Vec::new();

    for line in raw_code.lines() {
        ctxt.inc(); // increment line number

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let raw_op = parts.next().unwrap();
        let op = OpCode::from_str(raw_op);
        if op.is_err() {
            return Err(err!("{}.{}: Expected to find OpCode but found {}", ctxt.filename, ctxt.line, raw_op));
        }
        let op = op.unwrap();

        let op_type = OP_ARG_TYPES[op as usize];

        match op_type {
            OpArgT::Nil => {
                let line_is_over_chck = validate_line_is_over(&mut parts, op, &ctxt);
                if line_is_over_chck.is_err() {
                    return Err(line_is_over_chck.unwrap_err());
                }

                code.push(Code::Op(op));
            }
            OpArgT::Reg => {
                let reg = consume_reg(&mut parts, op, &ctxt);
                if reg.is_err() {
                    return Err(reg.unwrap_err());
                }
                let reg = reg.unwrap();

                let line_is_over_chck = validate_line_is_over(&mut parts, op, &ctxt);
                if line_is_over_chck.is_err() {
                    return Err(line_is_over_chck.unwrap_err());
                }

                code.push(Code::Op(op));
                code.push(Code::Reg(reg));
            }
            OpArgT::IntReg => {
                let int = consume_int(&mut parts, op, &ctxt);
                if int.is_err() {
                    return Err(int.unwrap_err());
                }
                let int = int.unwrap();

                let reg = consume_reg(&mut parts, op, &ctxt);
                if reg.is_err() {
                    return Err(reg.unwrap_err());
                }
                let reg = reg.unwrap();

                let line_is_over_chck = validate_line_is_over(&mut parts, op, &ctxt);
                if line_is_over_chck.is_err() {
                    return Err(line_is_over_chck.unwrap_err());
                }

                code.push(Code::Op(op));
                code.push(Code::Int(int));
                code.push(Code::Reg(reg));
            }
            OpArgT::RegReg => {
                let reg1 = consume_reg(&mut parts, op, &ctxt);
                if reg1.is_err() {
                    return Err(reg1.unwrap_err());
                }
                let reg1 = reg1.unwrap();

                let reg2 = consume_reg(&mut parts, op, &ctxt);
                if reg2.is_err() {
                    return Err(reg2.unwrap_err());
                }
                let reg2 = reg2.unwrap();

                let line_is_over_chck = validate_line_is_over(&mut parts, op, &ctxt);
                if line_is_over_chck.is_err() {
                    return Err(line_is_over_chck.unwrap_err());
                }

                code.push(Code::Op(op));
                code.push(Code::Reg(reg1));
                code.push(Code::Reg(reg2));
            }
        }
    }

    Ok(code)
}

fn consume_int(parts: &mut std::str::SplitWhitespace, op: OpCode, ctxt: &Ctxt) -> Result<i64, String> {
    let val = parts.next();
    if val.is_none() {
        return Err(err!("{}.{}: {} expected to find integer but found nothing", ctxt.filename, ctxt.line, op));
    }
    let val = val.unwrap();
    let val = i64::from_str(val);
    if val.is_err() {
        return Err(err!(
            "{}.{}: {} expected to find integer but got {}",
            ctxt.filename,
            ctxt.line,
            op,
            val.unwrap_err()
        ));
    }
    Ok(val.unwrap())
}

fn consume_reg(parts: &mut std::str::SplitWhitespace, op: OpCode, ctxt: &Ctxt) -> Result<u8, String> {
    let reg = parts.next();
    if reg.is_none() {
        return Err(err!(
            "{}.{}: {} expected to find register but found nothing",
            ctxt.filename,
            ctxt.line,
            op,
        ));
    }
    let reg = reg.unwrap();

    // make sure it has the r prefix
    if !reg.starts_with('r') {
        return Err(err!("{}.{}: {} expected to find register but got {}", ctxt.filename, ctxt.line, op, reg));
    }
    let reg = &reg[1..];

    let reg = u8::from_str(reg);
    if reg.is_err() {
        return Err(err!(
            "{}.{}: {} expected to find register but got {}",
            ctxt.filename,
            ctxt.line,
            op,
            reg.unwrap_err()
        ));
    }
    Ok(reg.unwrap())
}

fn validate_line_is_over(parts: &mut std::str::SplitWhitespace, op: OpCode, ctxt: &Ctxt) -> Result<(), String> {
    let next = parts.next();
    if next.is_some() {
        return Err(err!(
            "{}.{}: {} expected to find end of line but got {}",
            ctxt.filename,
            ctxt.line,
            op,
            next.unwrap()
        ));
    }
    Ok(())
}
