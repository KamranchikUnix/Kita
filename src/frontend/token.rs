#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal(String), Eof, Ident(String), Int(i64),
    Assign, Plus, Minus, Asterisk, Slash,
    Eq, NotEq, Lt, Gt,
    LParen, RParen, Comma,
    Function, Let, True, False, If, Then, Else, End, Return,
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "function" => Token::Function, "local" => Token::Let, "true" => Token::True,
        "false" => Token::False, "if" => Token::If, "then" => Token::Then,
        "else" => Token::Else, "end" => Token::End, "return" => Token::Return,
        _ => Token::Ident(ident.to_string()),
    }
}
