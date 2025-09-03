use super::token::{lookup_ident, Token};

pub struct Lexer {
    input: Vec<char>, position: usize, read_position: usize, ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self { input: input.chars().collect(), position: 0, read_position: 0, ch: '\0' };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        let tok = match self.ch {
            '=' => if self.peek() == '=' { self.read_char(); Token::Eq } else { Token::Assign },
            '~' | '!' => if self.peek() == '=' { self.read_char(); Token::NotEq } else { Token::Illegal(self.ch.to_string()) },
            '+' => Token::Plus, '-' => Token::Minus, '/' => Token::Slash, '*' => Token::Asterisk,
            '<' => Token::Lt, '>' => Token::Gt,
            ',' => Token::Comma, '(' => Token::LParen, ')' => Token::RParen,
            '\0' => Token::Eof,
            _ => {
                if self.ch.is_alphabetic() || self.ch == '_' {
                    let ident = self.read_identifier();
                    return lookup_ident(&ident);
                } else if self.ch.is_digit(10) {
                    return Token::Int(self.read_number());
                } else { Token::Illegal(self.ch.to_string()) }
            }
        };
        self.read_char();
        tok
    }
    
    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() { '\0' } else { self.input[self.read_position] };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&self) -> char { if self.read_position >= self.input.len() { '\0' } else { self.input[self.read_position] } }
    
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.ch.is_whitespace() { self.read_char(); continue; }
            if self.ch == '-' && self.peek() == '-' { while self.ch != '\n' && self.ch != '\0' { self.read_char(); } continue; }
            break;
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' { self.read_char(); }
        self.input[pos..self.position].iter().collect()
    }

    fn read_number(&mut self) -> i64 {
        let pos = self.position;
        while self.ch.is_digit(10) { self.read_char(); }
        self.input[pos..self.position].iter().collect::<String>().parse().unwrap_or(0)
    }
}
