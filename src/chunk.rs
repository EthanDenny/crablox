pub enum OpCode {
    OpReturn,
    OpConstant,
}

pub struct Chunk {
    pub codes: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<f64>,
}

impl Chunk {
    pub fn write_byte(&mut self, code: u8, line: usize) {
        self.codes.push(code);
        self.lines.push(line);
    }

    pub fn write_code(&mut self, code: OpCode, line: usize) {
        self.write_byte(code as u8, line);
    }

    pub fn write_constant(&mut self, constant_idx: u8) {
        self.codes.push(constant_idx);
        self.lines.push(*self.lines.last().or_else(|| Some(&0)).unwrap());
    }
    
    pub fn add_constant(&mut self, constant: f64) -> u8 {
        self.constants.push(constant);
        (self.constants.len() - 1) as u8
    }

    pub fn new() -> Chunk {
        Chunk {
            codes: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }
}
