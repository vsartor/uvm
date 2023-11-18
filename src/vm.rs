use std::io::Write;

use crate::{
    asm::{displayable_code, Code, OpCode},
    utils::{f2i, i2f},
};

const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 8 * 1024;
const CALL_STACK_SIZE: usize = 1 * 1024;

pub struct VM {
    regs: [i64; NUM_REGISTERS],
    stack: [i64; STACK_SIZE],
    call_stack: [usize; CALL_STACK_SIZE],
    code: Vec<Code>,
    pc: usize,
    sp: usize,
    csp: usize,
    cmp: i8,
    capture_output: bool,
}

struct StepResult {
    continue_running: bool,
    output: Option<String>,
}

impl VM {
    pub fn new(code: Vec<Code>) -> Self {
        Self {
            regs: [0; NUM_REGISTERS],
            stack: [0; STACK_SIZE],
            call_stack: [0; CALL_STACK_SIZE],
            code,
            pc: 0,
            sp: 0,
            csp: 0,
            cmp: 0,
            capture_output: false,
        }
    }

    pub fn capture_output(mut self) -> Self {
        self.capture_output = true;
        self
    }

    pub fn get_registers(&self) -> [i64; NUM_REGISTERS] {
        self.regs
    }

    pub fn get_registers_as_floats(&self) -> [f64; NUM_REGISTERS] {
        let mut regs = [0.0; NUM_REGISTERS];
        for i in 0..NUM_REGISTERS {
            regs[i] = i2f(self.regs[i]);
        }
        regs
    }

    pub fn get_cmp(&self) -> i8 {
        self.cmp
    }

    fn consume_op(&mut self) -> Result<OpCode, String> {
        match self.code[self.pc] {
            Code::Op(op) => {
                self.pc += 1;
                Ok(op)
            }
            _ => Err(err!("Expected an opcode, but got {} at {}", self.code[self.pc], self.pc)),
        }
    }

    fn consume_reg(&mut self) -> Result<usize, String> {
        match self.code[self.pc] {
            Code::Reg(reg) => {
                if reg as usize >= NUM_REGISTERS {
                    return Err(err!("Register {} out of bounds (>{})", reg, NUM_REGISTERS - 1));
                }

                self.pc += 1;
                Ok(reg as usize)
            }
            _ => Err(err!("Expected a register, but got {} at {}", self.code[self.pc], self.pc)),
        }
    }

    fn consume_int(&mut self) -> Result<i64, String> {
        match self.code[self.pc] {
            Code::Int(val) => {
                self.pc += 1;
                Ok(val)
            }
            _ => Err(err!("Expected an integer, but got {} at {}", self.code[self.pc], self.pc)),
        }
    }

    fn consume_addr(&mut self) -> Result<usize, String> {
        match self.code[self.pc] {
            Code::Addr(addr) => {
                self.pc += 1;
                Ok(addr)
            }
            _ => Err(err!("Expected an address, but got {} at {}", self.code[self.pc], self.pc)),
        }
    }

    fn consume_real(&mut self) -> Result<f64, String> {
        match self.code[self.pc] {
            Code::Real(val) => {
                self.pc += 1;
                Ok(val)
            }
            _ => Err(err!("Expected a real, but got {} at {}", self.code[self.pc], self.pc)),
        }
    }

    fn step(&mut self) -> Result<StepResult, String> {
        let mut res = StepResult {
            continue_running: true,
            output: None,
        };

        let op = self.consume_op();
        if op.is_err() {
            return Err(op.unwrap_err());
        }
        match op.unwrap() {
            OpCode::HALT => {
                res.continue_running = false;
                Ok(res)
            }
            OpCode::SET => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] = val;
                Ok(res)
            }
            OpCode::SETF => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };

                // store in the register as an integer
                let val = f2i(val);
                self.regs[reg] = val;
                Ok(res)
            }
            OpCode::MOV => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };
                self.regs[reg1] = self.regs[reg0];
                Ok(res)
            }
            OpCode::PUSH => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                if self.sp >= STACK_SIZE {
                    return Err(err!("Stack overflow"));
                }
                self.stack[self.sp] = self.regs[reg];
                self.sp += 1;
                Ok(res)
            }
            OpCode::PUSHL => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                if self.sp >= STACK_SIZE {
                    return Err(err!("Stack overflow"));
                }
                self.stack[self.sp] = val;
                self.sp += 1;
                Ok(res)
            }
            OpCode::POP => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                if self.sp == 0 {
                    return Err(err!("Stack underflow"));
                }
                self.sp -= 1;
                self.regs[reg] = self.stack[self.sp];
                Ok(res)
            }
            OpCode::PUSHRF => {
                let frame_size = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                // validate that the value is actually between 1 and NUM_REGISTERS-1
                if frame_size < 1 || frame_size as usize >= NUM_REGISTERS {
                    return Err(err!("PUSHRF received a register frame size of {} out of bounds", frame_size));
                }
                let frame_size = frame_size as usize;
                // validate we indeed have "frame_size" free spaces on stack
                if self.sp + frame_size >= STACK_SIZE {
                    return Err(err!("PUSHRF {}: stack overflow", frame_size));
                }

                // push the first `frame_size` registers from lowest to highest
                for reg in 0..frame_size {
                    self.stack[self.sp] = self.regs[reg];
                    self.sp += 1;
                }

                Ok(res)
            }
            OpCode::POPRF => {
                let frame_size = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                // validate that the value is actually between 1 and NUM_REGISTERS-1
                if frame_size < 1 || frame_size as usize >= NUM_REGISTERS {
                    return Err(err!("POPRF received a register frame size of {} out of bounds", frame_size));
                }
                let frame_size = frame_size as usize;
                // validate we indeed have "frame_size" filled spaces on stack
                if self.sp < frame_size {
                    return Err(err!("POPRF {}: stack underflow", frame_size));
                }

                // pop the first `frame_size` registers from highest to lowest (opposite of PUSHRF)
                for reg in (0..frame_size).rev() {
                    self.sp -= 1;
                    self.regs[reg] = self.stack[self.sp];
                }

                Ok(res)
            }
            OpCode::ADD => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg1] += self.regs[reg0];
                Ok(res)
            }
            OpCode::ADDL => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] += val;
                Ok(res)
            }
            OpCode::SUB => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg1] -= self.regs[reg0];
                Ok(res)
            }
            OpCode::SUBL => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] -= val;
                Ok(res)
            }
            OpCode::SUB2L => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] = val - self.regs[reg];
                Ok(res)
            }
            OpCode::MUL => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg1] *= self.regs[reg0];
                Ok(res)
            }
            OpCode::MULL => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] *= val;
                Ok(res)
            }
            OpCode::DIV => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg1] /= self.regs[reg0];
                Ok(res)
            }
            OpCode::DIVL => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] /= val;
                Ok(res)
            }
            OpCode::DIV2L => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] = val / self.regs[reg];
                Ok(res)
            }
            OpCode::MOD => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg1] %= self.regs[reg0];
                Ok(res)
            }
            OpCode::INC => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] += 1;
                Ok(res)
            }
            OpCode::DEC => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.regs[reg] -= 1;
                Ok(res)
            }
            OpCode::ADDF => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = i2f(self.regs[reg0]);
                let val2 = i2f(self.regs[reg1]);
                let val = val1 + val2;

                // store in the register as an integer
                self.regs[reg1] = f2i(val);
                Ok(res)
            }
            OpCode::ADDFL => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = val;
                let val2 = i2f(self.regs[reg]);
                let val = val1 + val2;

                // store in the register as an integer
                self.regs[reg] = f2i(val);
                Ok(res)
            }
            OpCode::SUBF => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = i2f(self.regs[reg0]);
                let val2 = i2f(self.regs[reg1]);
                let val = val2 - val1;

                // store in the register as an integer
                self.regs[reg1] = f2i(val);
                Ok(res)
            }
            OpCode::SUBFL => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = val;
                let val2 = i2f(self.regs[reg]);
                let val = val2 - val1;

                // store in the register as an integer
                self.regs[reg] = f2i(val);
                Ok(res)
            }
            OpCode::SUBF2L => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = val;
                let val2 = i2f(self.regs[reg]);
                let val = val1 - val2;

                // store in the register as an integer
                self.regs[reg] = f2i(val);
                Ok(res)
            }
            OpCode::MULF => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = i2f(self.regs[reg0]);
                let val2 = i2f(self.regs[reg1]);
                let val = val1 * val2;

                // store in the register as an integer
                self.regs[reg1] = f2i(val);
                Ok(res)
            }
            OpCode::MULFL => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = val;
                let val2 = i2f(self.regs[reg]);
                let val = val1 * val2;

                // store in the register as an integer
                self.regs[reg] = f2i(val);
                Ok(res)
            }
            OpCode::DIVF => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = i2f(self.regs[reg0]);
                let val2 = i2f(self.regs[reg1]);
                let val = val2 / val1;

                // store in the register as an integer
                self.regs[reg1] = f2i(val);
                Ok(res)
            }
            OpCode::DIVFL => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = val;
                let val2 = i2f(self.regs[reg]);
                let val = val2 / val1;

                // store in the register as an integer
                self.regs[reg] = f2i(val);
                Ok(res)
            }
            OpCode::DIVF2L => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = val;
                let val2 = i2f(self.regs[reg]);
                let val = val1 / val2;

                // store in the register as an integer
                self.regs[reg] = f2i(val);
                Ok(res)
            }
            OpCode::POW => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = i2f(self.regs[reg0]);
                let val2 = i2f(self.regs[reg1]);
                let val = val2.powf(val1);

                // store in the register as an integer
                self.regs[reg1] = f2i(val);
                Ok(res)
            }
            OpCode::POW2 => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = i2f(self.regs[reg0]);
                let val2 = i2f(self.regs[reg1]);
                let val = val1.powf(val2);

                // store in the register as an integer
                self.regs[reg1] = f2i(val);
                Ok(res)
            }
            OpCode::POWL => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = val;
                let val2 = i2f(self.regs[reg0]);
                let val = val2.powf(val1);

                // store in the register as an integer
                self.regs[reg0] = f2i(val);
                Ok(res)
            }
            OpCode::POW2L => {
                let val = {
                    let val = self.consume_real();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val1 = val;
                let val2 = i2f(self.regs[reg0]);
                let val = val1.powf(val2);

                // store in the register as an integer
                self.regs[reg0] = f2i(val);
                Ok(res)
            }
            OpCode::CEIL => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val = i2f(self.regs[reg]);
                if val > std::i64::MAX as f64 || val < std::i64::MIN as f64 {
                    return Err(err!("CEIL overflow"));
                }

                self.regs[reg] = val.ceil() as i64;
                Ok(res)
            }
            OpCode::FLOR => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };

                let val = i2f(self.regs[reg]);
                if val > std::i64::MAX as f64 || val < std::i64::MIN as f64 {
                    return Err(err!("FLOR overflow"));
                }

                self.regs[reg] = val.floor() as i64;
                Ok(res)
            }
            OpCode::CMP => {
                let reg0 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                let reg1 = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                self.cmp = match self.regs[reg1].cmp(&self.regs[reg0]) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                };
                Ok(res)
            }
            OpCode::CMPL => {
                let val = {
                    let val = self.consume_int();
                    if val.is_err() {
                        return Err(val.unwrap_err());
                    }
                    val.unwrap()
                };
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                // Set the flag to -1, 0, or 1 depending on the comparison result.
                self.cmp = match self.regs[reg].cmp(&val) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                };
                Ok(res)
            }
            OpCode::JMP => {
                let addr = {
                    let addr = self.consume_addr();
                    if addr.is_err() {
                        return Err(addr.unwrap_err());
                    }
                    addr.unwrap()
                };
                self.pc = addr;
                Ok(res)
            }
            OpCode::JEQ => {
                let addr = {
                    let addr = self.consume_addr();
                    if addr.is_err() {
                        return Err(addr.unwrap_err());
                    }
                    addr.unwrap()
                };
                if self.cmp == 0 {
                    self.pc = addr;
                }
                Ok(res)
            }
            OpCode::JLT => {
                let addr = {
                    let addr = self.consume_addr();
                    if addr.is_err() {
                        return Err(addr.unwrap_err());
                    }
                    addr.unwrap()
                };
                if self.cmp == -1 {
                    self.pc = addr;
                }
                Ok(res)
            }
            OpCode::JLE => {
                let addr = {
                    let addr = self.consume_addr();
                    if addr.is_err() {
                        return Err(addr.unwrap_err());
                    }
                    addr.unwrap()
                };
                if self.cmp <= 0 {
                    self.pc = addr;
                }
                Ok(res)
            }
            OpCode::JGT => {
                let addr = {
                    let addr = self.consume_addr();
                    if addr.is_err() {
                        return Err(addr.unwrap_err());
                    }
                    addr.unwrap()
                };
                if self.cmp == 1 {
                    self.pc = addr;
                }
                Ok(res)
            }
            OpCode::JGE => {
                let addr = {
                    let addr = self.consume_addr();
                    if addr.is_err() {
                        return Err(addr.unwrap_err());
                    }
                    addr.unwrap()
                };
                if self.cmp >= 0 {
                    self.pc = addr;
                }
                Ok(res)
            }
            OpCode::JNE => {
                let addr = {
                    let addr = self.consume_addr();
                    if addr.is_err() {
                        return Err(addr.unwrap_err());
                    }
                    addr.unwrap()
                };
                if self.cmp != 0 {
                    self.pc = addr;
                }
                Ok(res)
            }
            OpCode::CALL => {
                let addr = {
                    let addr = self.consume_addr();
                    if addr.is_err() {
                        return Err(addr.unwrap_err());
                    }
                    addr.unwrap()
                };
                if self.csp >= CALL_STACK_SIZE {
                    return Err(err!("Call stack overflow"));
                }
                self.call_stack[self.csp] = self.pc;
                self.csp += 1;
                self.pc = addr;
                Ok(res)
            }
            OpCode::RET => {
                if self.csp == 0 {
                    return Err(err!("Call stack underflow"));
                }
                self.csp -= 1;
                self.pc = self.call_stack[self.csp];
                Ok(res)
            }
            OpCode::DBGREG => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                res.output = Some(dbg!("r{} = {}", reg, self.regs[reg]));
                Ok(res)
            }
            OpCode::DBGREGF => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap() as usize
                };
                let val = i2f(self.regs[reg]);
                res.output = Some(dbg!("r{} = {}", reg, val));
                Ok(res)
            }
            OpCode::DBGREGS => {
                res.output = Some(dbg!("regs = {:?}", self.regs));
                Ok(res)
            }
        }
    }

    pub fn run(&mut self) -> Result<String, String> {
        let mut captured_output = String::new();

        loop {
            match self.step() {
                Ok(res) => {
                    if let Some(output) = res.output {
                        if !self.capture_output {
                            println!("{}", output);
                        } else {
                            captured_output.push_str(&output);
                            captured_output.push('\n');
                        }
                    }
                    if !res.continue_running {
                        return Ok(captured_output);
                    }
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }
    }

    pub fn debugger(&mut self) -> Result<String, String> {
        let mut wait_for_input = true;
        let mut allowed_to_run = false;
        let mut breakpoints: Vec<usize> = Vec::new();

        let (displayable_code, addr2idx, idx2addr) = displayable_code(&self.code);

        loop {
            // check if current PC is a breakpoint
            if breakpoints.contains(&self.pc) {
                if allowed_to_run {
                    // we just hit a breakpoint, so we need to stop
                    // if allowed_to_run was false, it means we already had hit this breakpoint
                    // and the user is just running inspectioning commands
                    println!("Breakpoint hit at address {}", self.pc);
                }
                wait_for_input = true;
            }

            if wait_for_input {
                print!("> ");
                std::io::stdout().flush().unwrap();
                let input = {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    input
                };

                let mut tokens = input.split_whitespace();
                let cmd = {
                    let cmd = tokens.next();
                    if cmd.is_none() {
                        continue;
                    }
                    cmd.unwrap()
                };

                match cmd {
                    "r" | "regs" => {
                        allowed_to_run = false;

                        println!("regs = {:?}", self.regs);
                    }
                    "st" | "stack" => {
                        allowed_to_run = false;

                        // get how many entries from the top of stack to print
                        let num_entries = {
                            let num_entries = tokens.next();
                            if num_entries.is_none() {
                                println!("Expected a number of entries to print");
                                continue;
                            }
                            let num_entries = num_entries.unwrap();
                            let num_entries = num_entries.parse::<usize>();
                            if num_entries.is_err() {
                                println!("Expected a valid number of entries to print");
                                continue;
                            }
                            num_entries.unwrap()
                        };

                        // print the stack with the top of stack first (i.e. in reverse order)
                        print!("SP = {}, Stack = [", self.sp);
                        let num_entries = std::cmp::min(num_entries, self.sp);

                        if num_entries == 0 {
                            println!("]");
                            continue;
                        }

                        for i in 0..num_entries {
                            let idx = self.sp - i - 1;
                            if i == num_entries - 1 {
                                if num_entries == self.sp {
                                    // showing all of stack, so represent this by a closed bracked
                                    println!("{}]", self.stack[idx]);
                                } else {
                                    // there is stack remaining, so represent this directly
                                    println!("{}, ...<{} hidden>]", self.stack[idx], self.sp - num_entries);
                                }
                            } else {
                                print!("{}, ", self.stack[idx]);
                            }
                        }
                    }
                    "s" | "step" => {
                        allowed_to_run = true;
                    }
                    "p" | "play" => {
                        allowed_to_run = true;
                        wait_for_input = false;
                    }
                    "x" | "exit" => {
                        return Ok("".to_string());
                    }
                    "bp" | "breakpoint" => {
                        allowed_to_run = false;

                        let addr = {
                            let addr = tokens.next();
                            if addr.is_none() {
                                println!("Expected an address");
                                continue;
                            }
                            let addr = addr.unwrap();
                            let addr = addr.parse::<usize>();
                            if addr.is_err() {
                                println!("Expected a valid address");
                                continue;
                            }
                            addr.unwrap()
                        };

                        if breakpoints.contains(&addr) {
                            println!("Breakpoint at {} was removed", addr);
                            breakpoints.retain(|&x| x != addr);
                        } else {
                            println!("Breakpoint set at address {}", addr);
                            breakpoints.push(addr);
                        }
                    }
                    "c" | "code" => {
                        allowed_to_run = false;

                        let window_size = {
                            let window_size = tokens.next();
                            if window_size.is_none() {
                                println!("Expected a window size");
                                continue;
                            }
                            let window_size = window_size.unwrap();
                            let window_size = window_size.parse::<usize>();
                            if window_size.is_err() {
                                println!("Expected a valid window size");
                                continue;
                            }
                            window_size.unwrap()
                        };

                        let current_idx = addr2idx[&self.pc];
                        let start_idx = if current_idx >= window_size {
                            current_idx - window_size
                        } else {
                            0
                        };
                        let end_idx = if current_idx + window_size >= displayable_code.len() {
                            displayable_code.len()
                        } else {
                            current_idx + window_size
                        };

                        for (idx, line) in displayable_code[start_idx..end_idx].iter().enumerate() {
                            if current_idx == start_idx + idx {
                                print!("⇨ ");
                            } else {
                                print!("  ");
                            }
                            println!("{:04} {}", idx2addr[&(start_idx + idx)], line);
                        }
                    }
                    _ => {
                        allowed_to_run = false;

                        println!("Unknown command: {}", cmd);
                        continue;
                    }
                }
            }

            if allowed_to_run {
                match self.step() {
                    Ok(res) => {
                        if let Some(output) = res.output {
                            println!("PROGRAM OUTPUT> {}", output);
                        }
                        if !res.continue_running {
                            println!("<PROGRAM HALTED>");
                            return Ok("".to_string());
                        }
                    }
                    Err(msg) => {
                        // TODO: Maybe try something smart here for debugging purposes?
                        println!("PROGRAM ERROR> {}", msg);
                        return Err(msg);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Code, OpCode};

    #[test]
    fn test_halt() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::HALT)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_ok());
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_set() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::SET), Code::Int(42), Code::Reg(0),
            Code::Op(OpCode::SET), Code::Int(-42), Code::Reg(1),
            Code::Op(OpCode::HALT)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_ok());
        assert_eq!(vm.regs[0], 42);
        assert_eq!(vm.regs[1], -42);
    }

    #[test]
    fn test_add() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::SET), Code::Int(2), Code::Reg(0),
            Code::Op(OpCode::SET), Code::Int(40), Code::Reg(1),
            Code::Op(OpCode::ADD), Code::Reg(0), Code::Reg(1),
            Code::Op(OpCode::HALT)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_ok());
        assert_eq!(vm.regs[0], 2);
        assert_eq!(vm.regs[1], 42);
    }

    #[test]
    fn test_dbgreg() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::SET), Code::Int(42), Code::Reg(0),
            Code::Op(OpCode::DBGREG), Code::Reg(0),
            Code::Op(OpCode::HALT)
        ];
        let mut vm = VM::new(code).capture_output();
        let res = vm.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), dbg!("r0 = 42\n"));
    }

    #[test]
    fn test_dbgregs() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::SET), Code::Int(42), Code::Reg(0),
            Code::Op(OpCode::SET), Code::Int(-42), Code::Reg(1),
            Code::Op(OpCode::DBGREGS),
            Code::Op(OpCode::HALT)
        ];
        let mut vm = VM::new(code).capture_output();
        let res = vm.run();
        assert!(res.is_ok());

        // Build the expected result string. The first two registers will have the
        // values of 42 and -42, respectively. The rest (up to NUM_REGISTERS) will be 0.
        let mut expected_result = dbg!("regs = [42, -42").to_string();
        for _ in 0..(NUM_REGISTERS - 2) {
            expected_result.push_str(", 0");
        }
        expected_result.push_str("]\n");

        assert_eq!(res.unwrap(), expected_result);
    }

    #[test]
    fn test_fails_on_int_as_opcode() {
        #[rustfmt::skip]
        let code = vec![
            Code::Int(42)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_err());
    }

    #[test]
    fn test_fails_on_int_as_reg() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::SET), Code::Int(42), Code::Int(0)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_err());
    }

    #[test]
    fn test_fails_on_reg_as_opcode() {
        #[rustfmt::skip]
        let code = vec![
            Code::Reg(0)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_err());
    }

    #[test]
    fn test_fails_on_reg_as_int() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::SET), Code::Reg(0), Code::Reg(0)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_err());
    }

    #[test]
    fn test_fails_on_op_as_int() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::SET), Code::Op(OpCode::HALT), Code::Reg(0)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_err());
    }

    #[test]
    fn test_fails_on_op_as_reg() {
        #[rustfmt::skip]
        let code = vec![
            Code::Op(OpCode::SET), Code::Reg(0), Code::Op(OpCode::HALT)
        ];
        let mut vm = VM::new(code);
        assert!(vm.run().is_err());
    }
}
