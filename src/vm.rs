use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::debug;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {}

impl VirtualMachine {
    pub fn interpret(chunk: &Chunk) -> InterpretResult {
        let mut ip = chunk.codes.iter().enumerate();

        loop {
            let next = ip.next();
            if next == None { return InterpretResult::Ok; }

            let idx = next.unwrap().0;
            let instruction = *next.unwrap().1;

            debug::disassemble_instruction(chunk, idx);
            
            match instruction {
                x if x == OpCode::Return as u8 => {
                    return InterpretResult::Ok;
                }
                x if x == OpCode::Constant as u8 => {
                    let next = ip.next();
                    if next == None { return InterpretResult::CompileError; }

                    let constant_idx = *next.unwrap().1 as usize;

                    let constant = chunk.constants[constant_idx];
                    println!("{}", constant);
                }
                x if x == OpCode::LongConstant as u8 => {
                    let mut constant_idx = 0;
                    let mut factor = 256 * 256;

                    for _ in 0..3 {
                        let next = ip.next();
                        if next == None { return InterpretResult::CompileError; }

                        constant_idx += *next.unwrap().1 as usize * factor;
                        factor /= 256;
                    }

                    let constant = chunk.constants[constant_idx as usize];
                    println!("{}", constant);
                }
                _ => {}
            }
        }
    }
}
