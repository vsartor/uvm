use crate::asm;

const NUM_REGISTERS: u8 = 16;

pub struct VM {
    regs: [i64; NUM_REGISTERS as usize],
    code: Vec<asm::Code>,
    pc: usize,
}

pub fn new(code: Vec<asm::Code>) -> VM {
    // If we ever get in a situation where multiple VMs need to use the same
    // code, we can just change the current implementation to reference code
    // instead of owning it. Doing it through ownership for now only because
    // it's simpler.
    VM {
        regs: [0; NUM_REGISTERS as usize],
        code,
        pc: 0,
    }
}

impl VM {
    fn consume_op(&mut self) -> Result<asm::OpCode, String> {
        match self.code[self.pc] {
            asm::Code::Op(op) => {
                self.pc += 1;
                Ok(op)
            }
            _ => Err(err!("Expected an opcode, but got {:?}", self.code[self.pc])),
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
            _ => Err(err!("Expected a register, but got {:?}", self.code[self.pc])),
        }
    }

    fn consume_int(&mut self) -> Result<i64, String> {
        match self.code[self.pc] {
            asm::Code::Int(val) => {
                self.pc += 1;
                Ok(val)
            }
            _ => Err(err!("Expected an integer, but got {:?}", self.code[self.pc])),
        }
    }

    fn step(&mut self) -> Result<bool, String> {
        // Returns true if the VM should continue running, false if it should
        // halt.
        match self.consume_op() {
            Ok(asm::OpCode::HALT) => Ok(false),
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
                Ok(true)
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
                Ok(true)
            }
            Ok(asm::OpCode::DBGREG) => {
                let reg = {
                    let reg = self.consume_reg();
                    if reg.is_err() {
                        return Err(reg.unwrap_err());
                    }
                    reg.unwrap()
                };
                dbg!("r{} = {}", reg, self.regs[reg as usize]);
                Ok(true)
            }
            Ok(asm::OpCode::DBGREGS) => {
                dbg!("regs = {:?}", self.regs);
                Ok(true)
            }
            Err(msg) => {
                return Err(msg);
            }
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            match self.step() {
                Ok(continue_running) => {
                    if !continue_running {
                        return Ok(());
                    }
                }
                Err(msg) => {
                    return Err(msg);
                }
            }
        }
    }
}
