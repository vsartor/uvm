#[macro_use]
mod log_macros;
mod asm;
mod vm;

fn main() {
    #[rustfmt::skip]
    let program: Vec<asm::Code> = vec![
        asm::Code::Op(asm::OpCode::SET), asm::Code::Int(15), asm::Code::Reg(0),
        asm::Code::Op(asm::OpCode::SET), asm::Code::Int(-4), asm::Code::Reg(1),
        asm::Code::Op(asm::OpCode::ADD), asm::Code::Reg(0), asm::Code::Reg(1),
        asm::Code::Op(asm::OpCode::DBGREG), asm::Code::Reg(1),
        asm::Code::Op(asm::OpCode::DBGREGS),
        asm::Code::Op(asm::OpCode::HALT),
    ];

    let mut vm = vm::new(program);

    let result = vm.run();
    if result.is_err() {
        println!("{}", result.unwrap_err());
    }
}
