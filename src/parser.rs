use std::{collections::HashMap, str::FromStr};

use crate::asm::{Code, OpArgT, OpCode, OP_ARG_TYPES};

struct Ctxt {
    filename: String,
    line: usize,
}

impl Ctxt {
    fn new(filename: String) -> Self {
        Self { filename, line: 0 }
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

    // hashmap where we store the labels and their corresponding "address"
    let mut labels: HashMap<String, usize> = std::collections::HashMap::new();
    // hashmap where we map where references to labels happened to the label being addressed
    let mut label_refs: HashMap<usize, String> = std::collections::HashMap::new();
    // after parsing the program, we substitute label_refs by the actual "address"

    let mut current_parent_label: String = "__beggining_of_program__".to_string();

    for line in raw_code.lines() {
        ctxt.inc(); // increment line number

        // Trim whitespace and ignore any comments (i.e. everything starting after //)
        let line = line.splitn(2, "//").next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let raw_op = parts.next().unwrap();

        // raw_op can either be an actual op or a label, so let's check if it's a label first
        // if it is a label, we'll skip to the next line
        if raw_op.ends_with(":") {
            if raw_op.starts_with(".") {
                // sublabel, so the actual label is the concatenation of the current parent label
                let label = format!("{}>{}", current_parent_label, &raw_op[1..raw_op.len() - 1]);
                if labels.contains_key(&label) {
                    // TODO: add unit test for this behavior
                    return Err(err!("{}.{}: Sublabel {} already defined", ctxt.filename, ctxt.line, label));
                }
                labels.insert(label, code.len());
            } else {
                // regular label
                let label = raw_op[..raw_op.len() - 1].to_string();
                if labels.contains_key(&label) {
                    // TODO: add unit test for this behavior
                    return Err(err!("{}.{}: Label {} already defined", ctxt.filename, ctxt.line, label));
                }
                labels.insert(label.clone(), code.len());
                current_parent_label = label;
            }
            continue;
        }

        let op = OpCode::from_str(raw_op);
        if op.is_err() {
            return Err(err!("{}.{}: Expected to find an OpCode but found {}", ctxt.filename, ctxt.line, raw_op));
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
            OpArgT::Addr => {
                // this should be a label while parsing
                // we'll remember that here we had this reference to a label and its address
                // and only at the end we'll make the substitution
                let label = parts.next();
                if label.is_none() {
                    // TODO: add unit test for this behavior
                    return Err(err!(
                        "{}.{}: {} expected to find a label but found nothing",
                        ctxt.filename,
                        ctxt.line,
                        op
                    ));
                }
                let label = label.unwrap();

                // handle sublabel behavior
                let label = if label.starts_with(".") {
                    format!("{}>{}", current_parent_label, &label[1..])
                } else {
                    label.to_string()
                };

                // note that currently code.len() will point to the operation that
                // takes in the addr, so to point to the addr itself we'll need a +1
                label_refs.insert(code.len() + 1, label);

                code.push(Code::Op(op));
                code.push(Code::Addr(0)); // placeholder
            }
            OpArgT::Int => {
                let int = {
                    let int = consume_int(&mut parts, op, &ctxt);
                    if int.is_err() {
                        return Err(int.unwrap_err());
                    }
                    int.unwrap()
                };

                let line_is_over_chck = validate_line_is_over(&mut parts, op, &ctxt);
                if line_is_over_chck.is_err() {
                    return Err(line_is_over_chck.unwrap_err());
                }

                code.push(Code::Op(op));
                code.push(Code::Int(int));
            }
            OpArgT::RealReg => {
                let val = {
                    let val = consume_real(&mut parts, op, &ctxt);
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };

                let reg = {
                    let reg = consume_reg(&mut parts, op, &ctxt);
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };

                let line_is_over_chck = validate_line_is_over(&mut parts, op, &ctxt);
                if line_is_over_chck.is_err() {
                    return Err(line_is_over_chck.unwrap_err());
                }

                code.push(Code::Op(op));
                code.push(Code::Real(val));
                code.push(Code::Reg(reg));
            }
        }
    }

    // Now, for each entry in label_refs, we'll substitute the label by its address
    for (addr, label) in label_refs {
        let label_addr = labels.get(&label);
        if label_addr.is_none() {
            // TODO: add unit test for this behavior
            return Err(err!("Reference to label {} at addr {} found but it's not defined", label, addr));
        }
        let label_addr = label_addr.unwrap();
        code[addr] = Code::Addr(*label_addr);
    }

    Ok(code)
}

fn consume_int(parts: &mut std::str::SplitWhitespace, op: OpCode, ctxt: &Ctxt) -> Result<i64, String> {
    let val = parts.next();
    if val.is_none() {
        return Err(err!(
            "{}.{}: {} expected to find an integer but found nothing",
            ctxt.filename,
            ctxt.line,
            op
        ));
    }
    let val = val.unwrap();
    let val = i64::from_str(val);
    if val.is_err() {
        return Err(err!(
            "{}.{}: {} expected to find an integer but got {}",
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
            "{}.{}: {} expected to find a register but found nothing",
            ctxt.filename,
            ctxt.line,
            op,
        ));
    }
    let reg = reg.unwrap();

    // make sure it has the r prefix
    if !reg.starts_with('r') {
        return Err(err!(
            "{}.{}: {} expected to find a register but got {}",
            ctxt.filename,
            ctxt.line,
            op,
            reg
        ));
    }
    let reg = &reg[1..];

    let reg = u8::from_str(reg);
    if reg.is_err() {
        return Err(err!(
            "{}.{}: {} expected to find a register but got {}",
            ctxt.filename,
            ctxt.line,
            op,
            reg.unwrap_err()
        ));
    }
    Ok(reg.unwrap())
}

fn consume_real(parts: &mut std::str::SplitWhitespace, op: OpCode, ctxt: &Ctxt) -> Result<f64, String> {
    let val = parts.next();
    if val.is_none() {
        return Err(err!("{}.{}: {} expected to find a real but found nothing", ctxt.filename, ctxt.line, op));
    }
    let val = val.unwrap();
    let val = f64::from_str(val);
    if val.is_err() {
        return Err(err!(
            "{}.{}: {} expected to find a real but got {}",
            ctxt.filename,
            ctxt.line,
            op,
            val.unwrap_err()
        ));
    }
    Ok(val.unwrap())
}

fn validate_line_is_over(parts: &mut std::str::SplitWhitespace, op: OpCode, ctxt: &Ctxt) -> Result<(), String> {
    let next = parts.next();
    if next.is_some() {
        Err(err!(
            "{}.{}: {} expected to find end of line but got {}",
            ctxt.filename,
            ctxt.line,
            op,
            next.unwrap()
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_ctxt() -> Ctxt {
        Ctxt::new("test".to_string())
    }

    #[test]
    fn test_parsing_halt_program() {
        let raw_code = "HALT".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());
        assert!(code.is_ok());
        assert_eq!(code.unwrap(), vec![Code::Op(OpCode::HALT)]);
    }

    #[test]
    fn test_parsing_halt_set_add_program() {
        #[rustfmt::skip]
        let raw_code = "
            SET 2 r0
            SET 40 r1
            ADD r0 r1
            HALT".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());

        #[rustfmt::skip]
        let expected_code = vec![
            Code::Op(OpCode::SET), Code::Int(2), Code::Reg(0),
            Code::Op(OpCode::SET), Code::Int(40), Code::Reg(1),
            Code::Op(OpCode::ADD), Code::Reg(0), Code::Reg(1),
            Code::Op(OpCode::HALT),
        ];

        assert!(code.is_ok());
        assert_eq!(code.unwrap(), expected_code);
    }

    #[test]
    fn shows_filename_and_lineno_of_error() {
        let raw_code = "HALT\nHALT\nSET 2 r-2".to_string();
        let code = parse_string(&raw_code, Ctxt::new("fff".to_string()));
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("fff.3"));
    }

    #[test]
    fn shows_opcode_on_reg_error() {
        let raw_code = "SET 0 0";
        let code = parse_string(&raw_code.to_string(), dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("SET"));
    }

    #[test]
    fn fails_on_register_as_integer() {
        let raw_code = "SET r2 r0".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("expected to find an integer but"));
    }

    #[test]
    fn fails_on_double_as_integer() {
        let raw_code = "SET 2.0 r0".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("expected to find an integer but"));
    }

    #[test]
    fn fails_on_register_without_r() {
        let raw_code = "SET 2 0".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("expected to find a register but"));
    }

    #[test]
    fn fails_on_register_with_only_an_r() {
        let raw_code = "SET 2 r".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("expected to find a register but got"));
    }

    #[test]
    fn fails_on_register_without_valid_integer() {
        let raw_code = "SET 2 r2.0".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("expected to find a register but got"));
    }

    #[test]
    fn fails_on_register_with_negative_integer() {
        let raw_code = "SET 2 r-2".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("expected to find a register but got"));
    }

    #[test]
    fn fails_on_unexpected_opcode() {
        let raw_code = "crymeariver";
        let code = parse_string(&raw_code.to_string(), dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("Expected to find an OpCode but"));
    }

    #[test]
    fn fails_on_too_many_args() {
        let raw_code = "HALT 2".to_string();
        let code = parse_string(&raw_code, dummy_ctxt());
        assert!(code.is_err());
        assert!(code.unwrap_err().contains("expected to find end of line but"));
    }
}
