pub mod chunk;
pub mod debug;
pub mod value;
pub mod vm;

use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::vm::VirtualMachine;

fn main() {
    let mut chunk = Chunk::new();

    let constant_idx = chunk.add_constant(1.2);
    chunk.write_constant(constant_idx, 123);

    let constant_idx = chunk.add_constant(3.4);
    chunk.write_constant(constant_idx, 123);

    chunk.write_code(OpCode::Add, 123);

    let constant_idx = chunk.add_constant(5.6);
    chunk.write_constant(constant_idx, 123);

    chunk.write_code(OpCode::Divide, 123);
    chunk.write_code(OpCode::Negate, 123);

    chunk.write_code(OpCode::Return, 123);

    VirtualMachine::interpret(&chunk);
}
