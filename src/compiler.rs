use crate::scanner::Scanner;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,
  
    Error, Eof
}

pub struct Token {
    pub token_type: TokenType,
    pub slice: String,
    pub line: usize,
}

pub fn compile(source: String) {
    let mut scanner = Scanner {
        source: source.clone(),
        start: 0,
        current: 0,
        line: 1,
    };

    let mut line = 0;
    loop {
        let token = scan_token(&mut scanner);

        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:2} '{}'", token.token_type as u8, token.slice); 

        if token.token_type == TokenType::Eof { break; }
    }
}

fn scan_token(scanner: &mut Scanner) -> Token {
    scanner.skip_whitespace();

    scanner.start = scanner.current;

    if scanner.is_at_end() {
        return scanner.make_token(TokenType::Eof);
    }

    let c = scanner.advance();

    match c {
        '(' => scanner.make_token(TokenType::LeftParen),
        ')' => scanner.make_token(TokenType::RightParen),
        '{' => scanner.make_token(TokenType::LeftBrace),
        '}' => scanner.make_token(TokenType::RightBrace),
        ';' => scanner.make_token(TokenType::Semicolon),
        ',' => scanner.make_token(TokenType::Comma),
        '.' => scanner.make_token(TokenType::Dot),
        '-' => scanner.make_token(TokenType::Minus),
        '+' => scanner.make_token(TokenType::Plus),
        '/' => scanner.make_token(TokenType::Slash),
        '*' => scanner.make_token(TokenType::Star),
        '!' => {
            if scanner.consume_if('=') {
                scanner.make_token(TokenType::BangEqual)
            } else {
                scanner.make_token(TokenType::Bang)
            }
        }
        '=' => {
            if scanner.consume_if('=') {
                scanner.make_token(TokenType::EqualEqual)
            } else {
                scanner.make_token(TokenType::Equal)
            }
        }
        '<' => {
            if scanner.consume_if('=') {
                scanner.make_token(TokenType::LessEqual)
            } else {
                scanner.make_token(TokenType::Less)
            }
        }
        '>' => {
            if scanner.consume_if('=') {
                scanner.make_token(TokenType::GreaterEqual)
            } else {
                scanner.make_token(TokenType::Greater)
            }
        }
        '"' => scanner.string(),
        '0'..='9' => scanner.number(),
        'a'..='z' | 'A'..='Z' | '_' => scanner.identifier(),
        _ => Scanner::error_token("Unexpected character."),
    }
}
