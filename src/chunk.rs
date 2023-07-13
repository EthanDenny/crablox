use crate::value::Value;

pub enum OpCode {
    Return,
    Constant,
    LongConstant,
}

pub struct Chunk {
    pub codes: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn write_byte(&mut self, code: u8, line: usize) {
        self.codes.push(code);
        self.lines.push(line);
    }

    pub fn write_code(&mut self, code: OpCode, line: usize) {
        self.write_byte(code as u8, line);
    }

    pub fn write_constant(&mut self, constant_idx: usize, line: usize) {
        if constant_idx <= 255 {
            self.write_code(OpCode::Constant, line);
            self.write_byte(constant_idx as u8, line);
        } else {
            self.write_code(OpCode::LongConstant, line);
            self.write_byte((constant_idx / (256 * 256)) as u8, line);
            self.write_byte(((constant_idx / 256) % 256) as u8, line);
            self.write_byte((constant_idx % 256) as u8, line);
        }
    }
    
    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    pub fn new() -> Chunk {
        Chunk {
            codes: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }
}
