use crate::asm::{Code, OpArgT, OpCode, OP_ARG_TYPES};

const UVM_BINARY_SIGNATURE: [u8; 8] = [0x2c, 0xdd, 0x59, 0x9b, 0x96, 0xe1, 0xbf, 0x50];
const UVM_BINARY_VERSION: u8 = 0x01;

pub fn serialize(code: &Vec<Code>) -> Result<Vec<u8>, String> {
    // start with randomly generated signature of 8 bytes PLUS a byte indicating the current version
    // so that when reading binaries we can check if they they are actually compatible uvm binaries
    let mut binary = Vec::from(UVM_BINARY_SIGNATURE);
    binary.push(UVM_BINARY_VERSION);

    let mut idx = 0;

    // we use variable sizes not to waste space (especially because opcodes and registers
    // are the most common "atoms" in the code):
    // 1 byte for opcode
    // 1 byte for register
    // 8 bytes for integer
    // 8 bytes for address

    while idx < code.len() {
        let op = match code[idx] {
            Code::Op(op) => op,
            _ => return Err(err!("Expected an opcode, but got {}", code[idx])),
        };
        let arg_t = OP_ARG_TYPES[op as usize];

        match arg_t {
            OpArgT::Nil => {
                binary.extend(op.to_le_bytes());
                idx += 1;
            }
            OpArgT::Reg => {
                binary.extend(op.to_le_bytes());

                let reg = match code[idx + 1] {
                    Code::Reg(reg) => reg,
                    _ => return Err(err!("Expected a register, but got {}", code[idx + 1])),
                };
                binary.extend(reg.to_le_bytes());
                idx += 2;
            }
            OpArgT::IntReg => {
                binary.extend(op.to_le_bytes());

                let int = match code[idx + 1] {
                    Code::Int(int) => int,
                    _ => return Err(err!("Expected an integer, but got {}", code[idx + 1])),
                };
                binary.extend(int.to_le_bytes());

                let reg = match code[idx + 2] {
                    Code::Reg(reg) => reg,
                    _ => return Err(err!("Expected a register, but got {}", code[idx + 2])),
                };
                binary.extend(reg.to_le_bytes());
                idx += 3;
            }
            OpArgT::RegReg => {
                binary.extend(op.to_le_bytes());

                let reg1 = match code[idx + 1] {
                    Code::Reg(reg) => reg,
                    _ => return Err(err!("Expected a register, but got {}", code[idx + 1])),
                };
                binary.extend(reg1.to_le_bytes());

                let reg2 = match code[idx + 2] {
                    Code::Reg(reg) => reg,
                    _ => return Err(err!("Expected a register, but got {}", code[idx + 2])),
                };
                binary.extend(reg2.to_le_bytes());
                idx += 3;
            }
            OpArgT::Addr => {
                binary.extend(op.to_le_bytes());

                let addr = match code[idx + 1] {
                    Code::Addr(addr) => addr,
                    _ => return Err(err!("Expected an address, but got {}", code[idx + 1])),
                };
                binary.extend(addr.to_le_bytes());
                idx += 2;
            }
            OpArgT::Int => {
                binary.extend(op.to_le_bytes());

                let int = match code[idx + 1] {
                    Code::Int(int) => int,
                    _ => return Err(err!("Expected an integer, but got {}", code[idx + 1])),
                };
                binary.extend(int.to_le_bytes());
                idx += 2;
            }
        }
    }

    Ok(binary)
}

pub fn deserialize(binary: Vec<u8>) -> Result<Vec<Code>, String> {
    let mut code = Vec::new();
    let mut idx = 0;

    // first we need to check signature
    if binary.len() < 9 {
        return Err(err!("Binary is too short to be a valid uvm binary ({} bytes)", binary.len()));
    }

    if binary[0..8] != UVM_BINARY_SIGNATURE {
        return Err(format!("Binary signature is invalid, this is not a UVM binary"));
    }

    // check version
    if binary[8] != UVM_BINARY_VERSION {
        return Err(err!(
            "Binary version is invalid, written with {} but current version is {}",
            binary[8],
            UVM_BINARY_VERSION
        ));
    }

    idx += 9;

    while idx < binary.len() {
        // get the opcode
        let op = match OpCode::from_le_bytes([binary[idx]]) {
            Some(op) => op,
            None => return Err(err!("Invalid opcode: {}", binary[idx])),
        };

        // figure out what we should be reading next
        let arg_t = OP_ARG_TYPES[op as usize];

        match arg_t {
            OpArgT::Nil => {
                code.push(Code::Op(op));
                idx += 1;
            }
            OpArgT::Reg => {
                let reg = u8::from_le_bytes([binary[idx + 1]]);
                code.push(Code::Op(op));
                code.push(Code::Reg(reg));
                idx += 2;
            }
            OpArgT::IntReg => {
                let int = i64::from_le_bytes([
                    binary[idx + 1],
                    binary[idx + 2],
                    binary[idx + 3],
                    binary[idx + 4],
                    binary[idx + 5],
                    binary[idx + 6],
                    binary[idx + 7],
                    binary[idx + 8],
                ]);
                let reg = u8::from_le_bytes([binary[idx + 9]]);
                code.push(Code::Op(op));
                code.push(Code::Int(int));
                code.push(Code::Reg(reg));
                idx += 10;
            }
            OpArgT::RegReg => {
                let reg1 = u8::from_le_bytes([binary[idx + 1]]);
                let reg2 = u8::from_le_bytes([binary[idx + 2]]);
                code.push(Code::Op(op));
                code.push(Code::Reg(reg1));
                code.push(Code::Reg(reg2));
                idx += 3;
            }
            OpArgT::Addr => {
                let addr = usize::from_le_bytes([
                    binary[idx + 1],
                    binary[idx + 2],
                    binary[idx + 3],
                    binary[idx + 4],
                    binary[idx + 5],
                    binary[idx + 6],
                    binary[idx + 7],
                    binary[idx + 8],
                ]);
                code.push(Code::Op(op));
                code.push(Code::Addr(addr));
                idx += 9;
            }
            OpArgT::Int => {
                let int = i64::from_le_bytes([
                    binary[idx + 1],
                    binary[idx + 2],
                    binary[idx + 3],
                    binary[idx + 4],
                    binary[idx + 5],
                    binary[idx + 6],
                    binary[idx + 7],
                    binary[idx + 8],
                ]);
                code.push(Code::Op(op));
                code.push(Code::Int(int));
                idx += 9;
            }
        }
    }

    Ok(code)
}
