use std::str::FromStr;

use crate::asm;

fn consume_int(parts: &mut std::str::SplitWhitespace) -> Result<i64, String> {
    let val = parts.next();
    if val.is_none() {
        return Err(err!("Expected to find integer but found nothing"));
    }
    let val = val.unwrap();
    let val = i64::from_str(val);
    if val.is_err() {
        return Err(err!("Expected to find integer but got {}", val.unwrap_err()));
    }
    Ok(val.unwrap())
}

fn consume_reg(parts: &mut std::str::SplitWhitespace) -> Result<u8, String> {
    let reg = parts.next();
    if reg.is_none() {
        return Err(err!("Expected to find register but found nothing"));
    }
    let reg = reg.unwrap().trim_start_matches('r'); // r0 -> 0
    let reg = u8::from_str(reg);
    if reg.is_err() {
        return Err(err!("Expected to find register but got {}", reg.unwrap_err()));
    }
    Ok(reg.unwrap())
}

pub fn parse_asm(raw_code: String) -> Result<Vec<asm::Code>, String> {
    let mut code = Vec::new();

    for line in raw_code.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let op = parts.next().unwrap();
        let op = asm::OpCode::from_str(op).unwrap();

        match op {
            asm::OpCode::HALT => code.push(asm::Code::Op(asm::OpCode::HALT)),
            asm::OpCode::SET => {
                let val = consume_int(&mut parts);
                if val.is_err() {
                    return Err(val.unwrap_err());
                }
                let val = val.unwrap();

                let reg = consume_reg(&mut parts);
                if reg.is_err() {
                    return Err(reg.unwrap_err());
                }
                let reg = reg.unwrap();

                code.push(asm::Code::Op(asm::OpCode::SET));
                code.push(asm::Code::Int(val));
                code.push(asm::Code::Reg(reg));
            }
            asm::OpCode::ADD => {
                let reg1 = consume_reg(&mut parts);
                if reg1.is_err() {
                    return Err(reg1.unwrap_err());
                }
                let reg1 = reg1.unwrap();

                let reg2 = consume_reg(&mut parts);
                if reg2.is_err() {
                    return Err(reg2.unwrap_err());
                }
                let reg2 = reg2.unwrap();

                code.push(asm::Code::Op(asm::OpCode::ADD));
                code.push(asm::Code::Reg(reg1));
                code.push(asm::Code::Reg(reg2));
            }
            asm::OpCode::DBGREG => {
                let reg = consume_reg(&mut parts);
                if reg.is_err() {
                    return Err(reg.unwrap_err());
                }
                let reg = reg.unwrap();

                code.push(asm::Code::Op(asm::OpCode::DBGREG));
                code.push(asm::Code::Reg(reg));
            }
            asm::OpCode::DBGREGS => code.push(asm::Code::Op(asm::OpCode::DBGREGS)),
        }
    }

    Ok(code)
}
