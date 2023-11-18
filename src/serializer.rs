use std::io::Write;

use crate::{
    asm::{Code, OpArgT, OpCode, OP_ARG_TYPES},
    parser::parse_file,
};

const UVM_SIGNATURE_LEN: usize = 15;
const UVM_BINARY_SIGNATURE: [u8; UVM_SIGNATURE_LEN] = [
    0x56, 0x69, 0x63, 0x74, 0x68, 0x6f, 0x72, 0x20, 0x69, 0x73, 0x20, 0x43, 0x30, 0x30, 0x4c,
];
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
    // 8 bytes for floats
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
            OpArgT::RealReg => {
                binary.extend(op.to_le_bytes());

                let real = match code[idx + 1] {
                    Code::Real(real) => real,
                    _ => return Err(err!("Expected a real, but got {}", code[idx + 1])),
                };
                binary.extend(real.to_le_bytes());

                let reg = match code[idx + 2] {
                    Code::Reg(reg) => reg,
                    _ => return Err(err!("Expected a register, but got {}", code[idx + 2])),
                };
                binary.extend(reg.to_le_bytes());
                idx += 3;
            }
        }
    }

    Ok(binary)
}

pub fn deserialize(binary: Vec<u8>) -> Result<Vec<Code>, String> {
    let mut code = Vec::new();
    let mut idx = 0;

    if binary.len() < UVM_SIGNATURE_LEN + 1 {
        return Err(err!("Binary is too short to be a valid uvm binary ({} bytes)", binary.len()));
    }

    if binary[..UVM_SIGNATURE_LEN] != UVM_BINARY_SIGNATURE {
        return Err(format!("Binary signature is invalid, this is not a UVM binary"));
    }

    // check version
    if binary[UVM_SIGNATURE_LEN] != UVM_BINARY_VERSION {
        return Err(err!(
            "Binary version is invalid, written with {} but current version is {}",
            binary[UVM_SIGNATURE_LEN],
            UVM_BINARY_VERSION
        ));
    }

    idx += UVM_SIGNATURE_LEN + 1;

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
            OpArgT::RealReg => {
                let real = f64::from_le_bytes([
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
                code.push(Code::Real(real));
                code.push(Code::Reg(reg));
                idx += 10;
            }
        }
    }

    Ok(code)
}

pub fn assemble(input_path: String, output_path: String) -> Result<(), String> {
    let code = {
        let parsed = parse_file(input_path);
        if parsed.is_err() {
            return Err(parsed.unwrap_err());
        }
        parsed.unwrap()
    };

    let serialized = {
        let serialized = serialize(&code);
        if serialized.is_err() {
            return Err(serialized.unwrap_err());
        }
        serialized.unwrap()
    };

    let mut file = {
        let file = std::fs::File::create(output_path);
        if file.is_err() {
            return Err(file.unwrap_err().to_string());
        }
        file.unwrap()
    };

    let write_op = file.write_all(&serialized);
    if write_op.is_err() {
        Err(write_op.unwrap_err().to_string())
    } else {
        Ok(())
    }
}

pub fn disassemble(input_path: String) -> Result<Vec<Code>, String> {
    let binary = {
        let binary = std::fs::read(input_path);
        if binary.is_err() {
            return Err(binary.unwrap_err().to_string());
        }
        binary.unwrap()
    };

    let deserialized = {
        let deserialized = deserialize(binary);
        if deserialized.is_err() {
            return Err(deserialized.unwrap_err());
        }
        deserialized.unwrap()
    };

    Ok(deserialized)
}
