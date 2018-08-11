use std::str::Chars;
use std::iter::Peekable;
use std::mem::replace;

use common::*;

pub struct Scanner<'a> {
    start: Chars<'a>,
    current: Peekable<Chars<'a>>,
    look_ahead: Option<char>,
    cur_len: usize,
    line: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    t_type: TokenType,
    lexeme: String,
    line: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            start: source.chars(),
            current: source.chars().peekable(),
            look_ahead: None,
            cur_len: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(la) = self.look_ahead {
            self.look_ahead = None;
            self.cur_len += 1;
            return Some(la);
        }
        let next = self.current.next();
        if next.is_some() {
            self.cur_len += 1;
        }
        next
    }

    fn make_token(&mut self, t_type: TokenType) -> Token {
        let lexeme = self.scan_lexeme();
        Token { lexeme, t_type, line: self.line }
    }

    fn scan_lexeme(&mut self) -> String {
        let mut lexeme = String::new();
        while self.cur_len > 0 {
            lexeme.push(self.start.next().unwrap());
            self.cur_len -= 1;
        }
        lexeme
    }

    fn match_char(&mut self, c: char) -> Token {
        use self::TokenType::*;

        match c {
            '(' => self.make_token(LeftParen),
            ')' => self.make_token(RightParen),
            '{' => self.make_token(LeftBrace),
            '}' => self.make_token(RightBrace),
            ';' => self.make_token(Semicolon),
            ',' => self.make_token(Comma),
            '.' => self.make_token(Dot),
            '-' => self.make_token(Minus),
            '+' => self.make_token(Plus),
            '/' => self.make_token(Slash),
            '*' => self.make_token(Star),
            '!' => self.possible_two_char_token(Bang, '=', BangEqual),
            '=' => self.possible_two_char_token(Equal, '=', EqualEqual),
            '>' => self.possible_two_char_token(Greater, '=', GreaterEqual),
            '<' => self.possible_two_char_token(Less, '=', LessEqual),
            _ => self.make_token(Error)
        }
    }

    fn possible_two_char_token(&mut self, cur_type: TokenType, char_to_match: char, possible_type: TokenType) -> Token {
        let t_type = if self.next_matches(char_to_match) {
            possible_type
        } else {
            cur_type
        };
        self.make_token(t_type)
    }

    fn next_matches(&mut self, c: char) -> bool {
        let next = self.peek();
        if let Some(n) = next {
            if c == n {
                self.advance();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn skip_whitespaces(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                match c {
                    ' ' | '\r' | '\t' => {
                        self.advance();
                    },
                    '\n' => {
                        self.line += 1;
                        self.advance();
                    }
                    '/' => {
                        if !self.skip_if_comment() {
                            break;
                        }
                    }
                    _ => break
                }
            } else {
                break;
            }
        }
        self.skip_lexeme();
    }

    fn skip_if_comment(&mut self) -> bool {
        if let Some('/') = self.peek_next() {
            while let Some(cc) = self.peek() {
                if cc == '\n' {
                    break;
                }
                self.advance();
            }
            true
        } else {
            false
        }
    }

    fn skip_lexeme(&mut self) {
        while self.cur_len > 0 {
            let _ = self.start.next();
            self.cur_len -= 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        if let Some(la) = self.look_ahead {
            return Some(la);
        }
        self.current.peek().map(|c| *c)
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.look_ahead.is_some() {
            return self.current.peek().map(|c| *c);
        }
        self.look_ahead = self.current.next();
        self.current.peek().map(|c| *c)
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespaces();
        let c = self.advance();
        match c {
            Some(c) => {
                Some(self.match_char(c))
            }
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use self::super::*;
    use super::TokenType::*;

    #[test]
    fn punctuation_scan() {
        let source = "/* != = +\n <  (){}\n!";
        let mut scanner = Scanner::new(source);

        assert_eq!(t("/", Slash, 1), scanner.next());
        assert_eq!(t("*", Star, 1), scanner.next());
        assert_eq!(t("!=", BangEqual, 1), scanner.next());
        assert_eq!(t("=", Equal, 1), scanner.next());
        assert_eq!(t("+", Plus, 1), scanner.next());

        assert_eq!(t("<", Less, 2), scanner.next());
        assert_eq!(t("(", LeftParen, 2), scanner.next());
        assert_eq!(t(")", RightParen, 2), scanner.next());
        assert_eq!(t("{", LeftBrace, 2), scanner.next());
        assert_eq!(t("}", RightBrace, 2), scanner.next());
        assert_eq!(t("!", Bang, 3), scanner.next());

        assert_eq!(None, scanner.next());
    }

    #[test]
    fn comments_scan() {
        let source = "+ // fr2f34f23f24;\n//\n/\n///";
        let mut scanner = Scanner::new(source);

        assert_eq!(t("+", Plus, 1), scanner.next());
        assert_eq!(t("/", Slash, 3), scanner.next());
        assert_eq!(None, scanner.next());
    }

    fn t(lexeme: &'static str, t_type: TokenType, line: usize) -> Option<Token> {
        Some(Token { lexeme: lexeme.to_string(), t_type, line })
    }
}