use crate::chunk::Chunk;
use crate::chunk::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut idx = 0;
    while idx < chunk.codes.len() {
        idx = disassemble_instruction(chunk, idx);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, idx: usize) -> usize {
    print!("{:04} ", idx);
    if idx > 0 && chunk.lines[idx] == chunk.lines[idx - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[idx]);
    }

    let code = chunk.codes.get(idx);
    if code == None { panic!("Index out of range in chunk"); }
    let code = *code.unwrap();

    match code {
        x if x == OpCode::Constant as u8 => {
            return constant_instruction("Constant", chunk, idx);
        }
        x if x == OpCode::LongConstant as u8 => {
            return long_constant_instruction("LongConstant", chunk, idx);
        }
        x if x == OpCode::Add as u8 => {
            return simple_instruction("Add", idx);
        }
        x if x == OpCode::Subtract as u8 => {
            return simple_instruction("Subtract", idx);
        }
        x if x == OpCode::Multiply as u8 => {
            return simple_instruction("Multiply", idx);
        }
        x if x == OpCode::Divide as u8 => {
            return simple_instruction("Divide", idx);
        }
        x if x == OpCode::Negate as u8 => {
            return simple_instruction("Negate", idx);
        }
        x if x == OpCode::Return as u8 => {
            return simple_instruction("Return", idx);
        }
        _ => {
            println!("Unknown opcode   {}", code);
            return idx + 1;
        }
    }
}

fn simple_instruction(name: &str, idx: usize) -> usize {
    println!("{}", name);
    return idx + 1;
}

fn constant_instruction(name: &str, chunk: &Chunk, idx: usize) -> usize {
    let constant_idx = *chunk.codes.get(idx + 1).unwrap();

    print!("{:<16} {:<5} '", name, constant_idx);
    println!("{}'", chunk.constants[constant_idx as usize]);

    return idx + 2;
}

fn long_constant_instruction(name: &str, chunk: &Chunk, idx: usize) -> usize {
    let constant_idx =
        *chunk.codes.get(idx + 1).unwrap() as usize * 256 * 256 +
        *chunk.codes.get(idx + 2).unwrap() as usize * 256 +
        *chunk.codes.get(idx + 3).unwrap() as usize;

    print!("{:<16} {:<5} '", name, constant_idx);
    println!("{}'", chunk.constants[constant_idx]);

    return idx + 4;
}
