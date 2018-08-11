use common::*;
use value::*;
use compiler::compile;

pub struct VM {
    ip: usize,
    stack: Vec<Value>
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

macro_rules! binary_stack_op {
    ($sel:ident, $name:ident) => {
        if let Some(r) = $sel.stack_pop() {
            if let Some(l) = $sel.stack_pop() {
                $sel.stack_push(l.$name(&r))
            } else {
                return RuntimeError
            }
        } else {
            return RuntimeError
        }
    };
}

impl VM {
    pub fn new() -> VM {
        VM { ip: 0, stack: Vec::new() }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        use common::Instruction::*;
        use self::InterpretResult::*;
        loop {
            match self.read_instruction(chunk) {
                Return => {
                    println!("{:?}", self.stack_pop());
                    return Ok
                },
                Constant(c) => {
                    let value = chunk.read_constant(*c);
                    self.stack_push(value.clone())
                },
                Negate => {
                    if let Some(v) = self.stack_pop() {
                        self.stack_push(v.negate());
                    } else {
                        return RuntimeError
                    }
                },
                Add => binary_stack_op!(self, add),
                Multiply => binary_stack_op!(self, multiply),
                Divide => binary_stack_op!(self, divide),
                Subtract => binary_stack_op!(self, subtract),
            }
        }
    }

    fn read_instruction<'a>(&mut self, chunk: &'a Chunk) -> &'a Instruction {
        self.ip += 1;
        &chunk.instructions[self.ip - 1]
    }

    fn stack_push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn stack_pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}

pub fn interpret_source(source: &str) -> InterpretResult {
    compile(source);
    InterpretResult::Ok
}