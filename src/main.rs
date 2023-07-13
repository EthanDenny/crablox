pub mod chunk;
pub mod debug;
pub mod value;
pub mod vm;

use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::vm::VirtualMachine;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.0);

    chunk.write_constant(constant, 123);

    for _ in 0..1000 {
        chunk.add_constant(1.2);
    }

    let constant = chunk.add_constant(2.0);

    chunk.write_constant(constant, 124);

    for _ in 0..65793 {
        chunk.add_constant(1.2);
    }

    let constant = chunk.add_constant(3.0);

    chunk.write_constant(constant, 125);
    chunk.write_code(OpCode::Return, 126);
    chunk.write_byte(12, 127);

    VirtualMachine::interpret(&chunk);
}
