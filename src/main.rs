pub mod chunk;
pub mod debug;

use crate::chunk::Chunk;
use crate::chunk::OpCode;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_code(OpCode::OpConstant, 123);
    chunk.write_constant(constant);
    chunk.write_code(OpCode::OpReturn, 123);

    debug::disassemble_chunk(&chunk, "test chunk");
}
