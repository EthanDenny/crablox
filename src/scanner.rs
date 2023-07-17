use crate::compiler::Token;
use crate::compiler::TokenType;

pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type: token_type,
            slice: self.source[self.start..self.current].to_string(),
            line: self.line,
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap_or('\0')
    }

    pub fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    pub fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    pub fn consume_if(&mut self, c: char) -> bool {
        if self.is_at_end() { return false; }
        if self.peek() != c { return false; }
        self.current += 1;
        true
    }

    pub fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => { self.advance(); }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek_next() == '/' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => { return; }
            }
        }
    }

    pub fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }
    
        if self.is_at_end() {
            Scanner::error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    pub fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    pub fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        return self.make_token(self.identifier_type());
    }

    pub fn error_token(message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            slice: message.to_string(),
            line: 1,
        }
    }

    fn identifier_type(&self) -> TokenType {
        match self.source.chars().nth(self.start).unwrap() {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' if self.current - self.start > 1 => {
                match self.source.chars().nth(self.start + 1).unwrap() {
                    'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                    'o' => self.check_keyword(2, 1, "r", TokenType::For),
                    'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                    _ => TokenType::Identifier,
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' if self.current - self.start > 1 => {
                match self.source.chars().nth(self.start + 1).unwrap() {
                    'h' => self.check_keyword(2, 2, "is", TokenType::This),
                    'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                    _ => TokenType::Identifier,
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + length &&
            &self.source[start..start+length] == rest {
            token_type
        } else {
            TokenType::Identifier
        }
    }
}
