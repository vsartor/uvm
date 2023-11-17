use crate::asm;

const NUM_REGISTERS: u8 = 16;

pub struct VM {
    regs: [i64; NUM_REGISTERS as usize],
    code: Vec<asm::Code>,
    pc: usize,
    cmp: i8,
    step_by_step: bool,
    capture_output: bool,
}

struct StepResult {
    continue_running: bool,
    output: Option<String>,
}

impl VM {
    pub fn new(code: Vec<asm::Code>) -> Self {
        Self {
            regs: [0; NUM_REGISTERS as usize],
            code,
            pc: 0,
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

    pub fn get_registers(&self) -> [i64; NUM_REGISTERS as usize] {
        self.regs
    }

    fn consume_op(&mut self) -> Result<asm::OpCode, String> {
        match self.code[self.pc] {
            asm::Code::Op(op) => {
                self.pc += 1;
                Ok(op)
            }
            _ => Err(err!("Expected an opcode, but got {} at {}", self.code[self.pc], self.pc)),
        }
    }

    fn consume_reg(&mut self) -> Result<u8, String> {
        match self.code[self.pc] {
            asm::Code::Reg(reg) => {
                if reg >= NUM_REGISTERS {
                    return Err(err!("Register {} out of bounds (>{})", reg, NUM_REGISTERS - 1));
                }

                self.pc += 1;
                Ok(reg)
            }
            _ => Err(err!("Expected a register, but got {} at {}", self.code[self.pc], self.pc)),
        }
    }

    fn consume_int(&mut self) -> Result<i64, String> {
        match self.code[self.pc] {
            asm::Code::Int(val) => {
                self.pc += 1;
                Ok(val)
            }
            _ => Err(err!("Expected an integer, but got {} at {}", self.code[self.pc], self.pc)),
        }
    }

    fn consume_addr(&mut self) -> Result<usize, String> {
        match self.code[self.pc] {
            asm::Code::Addr(addr) => {
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

        // Returns true if the VM should continue running, false if it should
        // halt.
        match self.consume_op() {
            Ok(asm::OpCode::HALT) => {
                res.continue_running = false;
                Ok(res)
            }
            Ok(asm::OpCode::SET) => {
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
                self.regs[reg as usize] = val;
                Ok(res)
            }
            Ok(asm::OpCode::ADD) => {
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
                self.regs[reg1 as usize] += self.regs[reg0 as usize];
                Ok(res)
            }
            Ok(asm::OpCode::SUB) => {
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
                self.regs[reg1 as usize] -= self.regs[reg0 as usize];
                Ok(res)
            }
            Ok(asm::OpCode::MUL) => {
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
                self.regs[reg1 as usize] *= self.regs[reg0 as usize];
                Ok(res)
            }
            Ok(asm::OpCode::DIV) => {
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
                self.regs[reg1 as usize] /= self.regs[reg0 as usize];
                Ok(res)
            }
            Ok(asm::OpCode::MOD) => {
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
                self.regs[reg1 as usize] %= self.regs[reg0 as usize];
                Ok(res)
            }
            Ok(asm::OpCode::CMPL) => {
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
                self.cmp = match self.regs[reg as usize].cmp(&val) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                };
                Ok(res)
            }
            Ok(asm::OpCode::JMP) => {
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
            Ok(asm::OpCode::JEQ) => {
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
            Ok(asm::OpCode::DBGREG) => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                res.output = Some(dbg!("r{} = {}", reg, self.regs[reg as usize]));
                Ok(res)
            }
            Ok(asm::OpCode::DBGREGS) => {
                res.output = Some(dbg!("regs = {:?}", self.regs));
                Ok(res)
            }
            Err(msg) => {
                return Err(msg);
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
    use asm::{Code, OpCode};

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
