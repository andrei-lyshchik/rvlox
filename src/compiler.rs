use common::*;
use scanner::*;
use value::*;

pub fn compile(source: &str) {
    let chunk = compile_to_chunk(source);

    chunk.disassemble();
}

fn compile_to_chunk(source: &str) -> Chunk {
    let scanner = Scanner::new(source);
    let mut chunk = Chunk::new();
    {
        let mut compiler = Compiler::new(scanner, &mut chunk);
        compiler.expression();
        compiler.finish_compiler();
    }

    chunk
}

pub struct Compiler<'a, 'b> {
    scanner: Scanner<'a>,
    current: Option<Token>,
    previous: Option<Token>,
    errors: Vec<Error>,
    panic_mode: bool,
    chunk: &'b mut Chunk,
    last_token_line: usize,
}

pub struct Error {
    location: ErrorLocation,
    msg: String,
}

pub enum ErrorLocation {
    Token(Token),
    AtTheEnd,
}

#[derive(PartialEq, Clone, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl<'a, 'b> Compiler<'a, 'b> {
    fn new(mut scanner: Scanner<'a>, chunk: &'b mut Chunk) -> Compiler<'a, 'b> {
        let current = scanner.next();
        Compiler {
            scanner,
            current,
            previous: None,
            errors: Vec::new(),
            panic_mode: false,
            chunk,
            last_token_line: 0,
        }
    }

    fn finish_compiler(&mut self) {
        self.emit_instruction_for_last_token(Instruction::Return);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        if let Some(token) = self.previous() {
            self.prefix_rule(&token);

            while let Some(current_token) = self.current() {
                if current_token.t_type.precedence() < precedence {
                    break;
                }

                self.advance();
                let previous = self.previous().unwrap();
                self.infix_rule(&previous);
            }
        }
    }

    fn prefix_rule(&mut self, token: &Token) {
        use scanner::TokenType::*;
        match token.t_type {
            LeftParen => self.grouping(),
            Minus => self.unary(token),
            Number(d) => self.number(d, token),
            _ => self.error("Expect expression", token),
        }
    }

    fn infix_rule(&mut self, token: &Token) {
        use scanner::TokenType::*;
        match token.t_type {
            Minus => self.binary(token),
            Plus => self.binary(token),
            Star => self.binary(token),
            Slash => self.binary(token),
            _ => panic!(
                "Can't invoke infix rule on this token type: {:?}",
                token.t_type
            ),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(
            TokenType::RightParen,
            "Expect to have ')' at the end of grouping expression",
        );
    }

    fn unary(&mut self, op_token: &Token) {
        use scanner::TokenType::*;

        self.expression();

        match op_token.t_type {
            Minus => self.emit_instruction(Instruction::Negate, op_token),
            _ => panic!(
                "Can not invoke 'unary' for token type: {:?}",
                op_token.t_type
            ),
        }
    }

    fn number(&mut self, number_val: f64, token: &Token) {
        let constant = self.chunk.add_constant(Value::Double(number_val));
        self.emit_instruction(Instruction::Constant(constant), token);
    }

    fn binary(&mut self, token: &Token) {
        use scanner::TokenType::*;

        let op_type = &token.t_type;

        self.parse_precedence(op_type.precedence().next());

        match op_type {
            Plus => self.emit_instruction_for_last_token(Instruction::Add),
            Minus => self.emit_instruction_for_last_token(Instruction::Subtract),
            Star => self.emit_instruction_for_last_token(Instruction::Multiply),
            Slash => self.emit_instruction_for_last_token(Instruction::Divide),
            _ => panic!("Can not invoke 'binary' for token type: {:?}", op_type),
        }
    }

    fn emit_instruction(&mut self, instruction: Instruction, token: &Token) {
        self.chunk.add_instruction(instruction, token.line);
    }

    fn emit_instruction_for_last_token(&mut self, instruction: Instruction) {
        self.chunk
            .add_instruction(instruction, self.last_token_line);
    }

    fn consume(&mut self, t_type: TokenType, error_msg: &'static str) {
        if let Some(current) = self.current() {
            if current.t_type == t_type {
                self.advance();
            } else {
                self.error(error_msg, &current);
            }
        } else {
            self.error_at_the_end(error_msg);
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        if let Some(ref t) = self.previous {
            self.last_token_line = t.line;
        }

        loop {
            self.current = self.scanner.next();
            if let Some(t) = self.current() {
                match t.t_type {
                    TokenType::Error(e) => self.error(e, &t),
                    _ => break,
                }
            } else {
                break;
            }
        }
    }

    fn error(&mut self, error_msg: &'static str, token: &Token) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        self.errors.push(Error::new(token.clone(), error_msg));
    }

    fn error_at_the_end(&mut self, error_msg: &'static str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        self.errors.push(Error::new_at_the_end(error_msg));
    }

    fn previous(&self) -> Option<Token> {
        self.previous.clone()
    }

    fn current(&self) -> Option<Token> {
        self.current.clone()
    }
}

impl Error {
    fn new(token: Token, msg: &'static str) -> Error {
        Error {
            location: ErrorLocation::Token(token),
            msg: msg.to_string(),
        }
    }

    fn new_at_the_end(msg: &'static str) -> Error {
        Error {
            location: ErrorLocation::AtTheEnd,
            msg: msg.to_string(),
        }
    }
}

impl Precedence {
    fn next(&self) -> Precedence {
        use self::Precedence::*;
        match self {
            None => Assignment,
            Assignment => Or,
            Or => And,
            And => Equality,
            Equality => Comparison,
            Comparison => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Call,
            Call => Primary,
            Primary => Primary,
        }
    }
}

trait ParseRule {
    fn precedence(&self) -> Precedence;
}

impl ParseRule for TokenType {
    fn precedence(&self) -> Precedence {
        use self::Precedence::*;
        use scanner::TokenType::*;
        match self {
            LeftParen => Call,
            Dot => Call,
            Minus => Term,
            Plus => Term,
            Slash => Factor,
            Star => Factor,
            BangEqual => Equality,
            EqualEqual => Equality,
            Greater => Comparison,
            GreaterEqual => Comparison,
            Less => Comparison,
            LessEqual => Comparison,
            TokenType::And => Precedence::And,
            TokenType::Or => Precedence::Or,
            _ => Precedence::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use self::super::*;
    use common::Instruction::*;

    #[test]
    fn simple_binary() {
        check_binary(1.0, 2.0, '+');
        check_binary(40.0, 32323.12, '-');
        check_binary(2132.0, 332.0, '/');
        check_binary(323.323, 0.32, '*');
    }

    #[test]
    fn binary_assoc() {
        check_binary_assoc(1.1, 3.2, 4.2, '+');
        check_binary_assoc(21.3, 23.1, 3.323, '-');
        check_binary_assoc(323.21, 3244.0, 3656.2, '*');
        check_binary_assoc(0.324, 345.1, 45.4, '/');
    }

    #[test]
    fn precedences() {
        check(
            "1 - 2 * 3",
            vec![c(0), c(1), c(2), Multiply, Subtract],
            vec![1.0, 2.0, 3.0]
        );

        check(
            "1 + 4 / 2",
            vec![c(0), c(1), c(2), Divide, Add],
            vec![1.0, 4.0, 2.0]
        );

        check(
            "2 * 3 + 4 / 5",
            vec![c(0), c(1), Multiply, c(2), c(3), Divide, Add],
            vec![2.0, 3.0, 4.0, 5.0]
        );
    }

    #[test]
    fn groupings() {
        check(
            "(1 + 2) * (3 - 4)",
            vec![c(0), c(1), Add, c(2), c(3), Subtract, Multiply],
            vec![1.0, 2.0, 3.0, 4.0]
        );

        check(
            "(((1 + 3) * 4) + 2) * 5",
            vec![c(0), c(1), Add, c(2), Multiply, c(3), Add, c(4), Multiply],
            vec![1.0, 3.0, 4.0, 2.0, 5.0]
        );
    }

    fn check_binary(lhs: f64, rhs: f64, op: char) {
        let source = format!("{} {} {}", lhs, op, rhs);

        let op_instruction = instruction_by_char_op(op);

        let instructions = vec![
            c(0),
            c(1),
            op_instruction,
        ];
        let constants = vec![lhs, rhs];

        check(&source, instructions, constants);
    }

    fn instruction_by_char_op(op: char) -> Instruction {
        match op {
            '+' => Add,
            '-' => Subtract,
            '*' => Multiply,
            '/' => Divide,
            _ => panic!("should use binary ops only here"),
        }
    }

    fn check_binary_assoc(n1: f64, n2: f64, n3: f64, op: char) {

        let source = format!("{} {} {} {} {}", n1, op, n2, op, n3);

        let op_instruction = instruction_by_char_op(op);

        let instructions = vec![
            c(0),
            c(1),
            op_instruction.clone(),
            c(2),
            op_instruction.clone(),
        ];

        let constants = vec![n1, n2, n3];

        check(&source, instructions, constants);
    }

    fn check(
        source: &str,
        instructions_without_line: Vec<Instruction>,
        double_constants: Vec<f64>,
    ) {
        let compiled = compile_to_chunk(source);

        let mut instructions_with_lines = Vec::new();
        for i in instructions_without_line {
            instructions_with_lines.push(InstructionWithLine(i, 1));
        }
        instructions_with_lines.push(InstructionWithLine(Instruction::Return, 1));

        let constants: Vec<Value> = double_constants.iter().map(|f| Value::Double(*f)).collect();

        assert_eq!(instructions_with_lines, compiled.instructions);
        assert_eq!(constants, compiled.constants);
    }

    fn c(i: usize) -> Instruction {
        Constant(i)
    }

}
