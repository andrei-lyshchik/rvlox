use value::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Return,
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub struct InstructionWithLine(pub Instruction, pub usize);

pub struct Chunk {
    pub instructions: Vec<InstructionWithLine>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, oc: Instruction, line: usize) {
        self.instructions.push(InstructionWithLine(oc, line))
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
