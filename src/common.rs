use value::Value;

#[derive(Debug)]
pub enum Instruction {
    Return,
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { instructions: Vec::new(), constants: Vec::new() }
    }

    pub fn add_instruction(&mut self, oc: Instruction) {
        self.instructions.push(oc)
    }

    pub fn add_constant(&mut self, c: Value) -> usize {
        self.constants.push(c);
        self.constants.len() - 1
    }

    pub fn read_constant(&self, i: usize) -> &Value {
        &self.constants[i]
    }
}

impl Chunk {
    pub fn disassemble(&self) {
        for (i, inc) in self.instructions.iter().enumerate() {
            println!("{} {:?}", i, inc);
        }
    }
}