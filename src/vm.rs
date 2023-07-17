use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::debug;
use crate::value::Value;

const DEBUG_TRACE_EXECUTION : bool = true;

#[derive(PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine {}

impl VirtualMachine {
    pub fn interpret(chunk: &Chunk) -> InterpretResult {
        let mut stack : Vec<Value> = Vec::new();
        let mut ip = chunk.codes.iter().enumerate();

        loop {
            let next = ip.next();
            if next == None { return InterpretResult::Ok; }

            let idx = next.unwrap().0;
            let instruction = *next.unwrap().1;

            if DEBUG_TRACE_EXECUTION {
                print!("          ");
                stack.iter().for_each(|slot| print!("[ {} ]", slot));
                print!("\n");

                debug::disassemble_instruction(chunk, idx);
            }
            
            match instruction {
                x if x == OpCode::Constant as u8 => {
                    let next = ip.next();
                    if next == None { return InterpretResult::CompileError; }

                    let constant_idx = *next.unwrap().1 as usize;

                    let constant = chunk.constants[constant_idx];
                    stack.push(constant);
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
                    stack.push(constant);
                }
                x if x == OpCode::Add as u8 => {
                    VirtualMachine::binary_op(&mut stack, |a, b| a + b);
                }
                x if x == OpCode::Subtract as u8 => {
                    VirtualMachine::binary_op(&mut stack, |a, b| a - b);
                }
                x if x == OpCode::Multiply as u8 => {
                    VirtualMachine::binary_op(&mut stack, |a, b| a * b);
                }
                x if x == OpCode::Divide as u8 => {
                    VirtualMachine::binary_op(&mut stack, |a, b| a / b);
                }
                x if x == OpCode::Negate as u8 => {
                    let v = stack.pop().unwrap();
                    stack.push(-v);
                }
                x if x == OpCode::Return as u8 => {
                    println!("{}", stack.pop().unwrap());
                    return InterpretResult::Ok;
                }
                _ => {}
            }
        }
    }

    fn binary_op(stack: &mut Vec<Value>, f: impl Fn(Value, Value) -> Value) {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(f(a, b));
    }
}
