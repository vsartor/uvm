use crate::asm::{Code, OpCode};

const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 256;

pub struct VM {
    regs: [i64; NUM_REGISTERS],
    stack: [i64; STACK_SIZE],
    code: Vec<Code>,
    pc: usize,
    sp: usize,
    cmp: i8,
    step_by_step: bool,
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
            code,
            pc: 0,
            sp: 0,
            cmp: 0,
            step_by_step: false,
            capture_output: false,
        }
    }

    pub fn capture_output(mut self) -> Self {
        self.capture_output = true;
        self
    }

    pub fn step_by_step(mut self) -> Self {
        self.step_by_step = true;
        self
    }

    pub fn get_registers(&self) -> [i64; NUM_REGISTERS] {
        self.regs
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
                println!("{}", self.sp);
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
            OpCode::SUBLA => {
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
            OpCode::SUBLB => {
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
            OpCode::DIVLA => {
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
            OpCode::DIVLB => {
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
            OpCode::DBGREGS => {
                res.output = Some(dbg!("regs = {:?}", self.regs));
                Ok(res)
            }
        }
    }

    pub fn run(&mut self) -> Result<String, String> {
        let mut captured_output = String::new();

        loop {
            if self.step_by_step {
                // TODO: do a decent debugger here jfc
                println!("{}", dbg!("pc={} | cmp={} | regs={:?}", self.pc, self.cmp, self.regs));
                println!("Press ENTER to continue...");
                let mut input = String::new();
                let _ = std::io::stdin().read_line(&mut input);
            }

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
