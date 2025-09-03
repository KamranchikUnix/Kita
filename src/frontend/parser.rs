use super::{ast::*, lexer::Lexer, token::Token};
use std::mem;

// 'Prefix' variant removed
#[derive(PartialEq, PartialOrd)]
enum Precedence { Lowest, Equals, LessGreater, Sum, Product, Call }

pub struct Parser {
    lexer: Lexer, current_token: Token, peek_token: Token, pub errors: Vec<String>,
}

// ... the rest of the file is identical to the previous version ...
// (You only need to change the Precedence enum at the top)
impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Self { lexer, current_token: Token::Eof, peek_token: Token::Eof, errors: vec![] };
        p.next_token(); p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();
        while self.current_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() { program.push(stmt); }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    
    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek_is_ident() { return None; }
        let name = if let Token::Ident(n) = self.current_token.clone() { n } else { return None; };
        if !self.expect_peek(Token::Assign) { return None; }
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        Some(Statement::Let { name, value })
    }
    
    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest)?;
        Some(Statement::Return(return_value))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        Some(Statement::Expression(expression))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut statements = Vec::new();
        self.next_token();
        while !matches!(self.current_token, Token::End | Token::Else | Token::Eof) {
            if let Some(stmt) = self.parse_statement() { statements.push(stmt); }
            self.next_token();
        }
        statements
    }
    
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_exp = match self.current_token.clone() {
            Token::Ident(name) => Expression::Identifier(name),
            Token::Int(val) => Expression::IntegerLiteral(val),
            Token::True => Expression::Boolean(true),
            Token::False => Expression::Boolean(false),
            Token::LParen => { self.next_token(); let exp = self.parse_expression(Precedence::Lowest)?; if !self.expect_peek(Token::RParen) { return None; } exp },
            Token::If => self.parse_if_expression()?,
            _ => { self.errors.push(format!("No prefix parse function for {:?}", self.current_token)); return None; }
        };
        
        while precedence < self.peek_precedence() {
            match self.peek_token {
                Token::LParen => { self.next_token(); left_exp = self.parse_call_expression(left_exp)?; },
                _ => { self.next_token(); left_exp = self.parse_infix_expression(left_exp)?; }
            }
        }
        Some(left_exp)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let op = self.current_token.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Some(Expression::Infix { op, left: Box::new(left), right: Box::new(right) })
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;
        if !self.expect_peek(Token::Then) { return None; }
        let consequence = self.parse_block_statement();
        let alternative = if self.current_token == Token::Else { Some(self.parse_block_statement()) } else { None };
        if !matches!(self.current_token, Token::End) { self.errors.push(format!("Expected 'end' to close 'if' block, got {:?}", self.current_token)); return None; }
        Some(Expression::If { condition: Box::new(condition), consequence, alternative })
    }
    
    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_call_arguments()?;
        Some(Expression::Call { function: Box::new(function), arguments })
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut args = Vec::new();
        if self.peek_token == Token::RParen { self.next_token(); return Some(args); }
        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest)?);
        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::Lowest)?);
        }
        if !self.expect_peek(Token::RParen) { return None; }
        Some(args)
    }

    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::Lt | Token::Gt => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
    fn peek_precedence(&self) -> Precedence { Self::token_to_precedence(&self.peek_token) }
    fn cur_precedence(&self) -> Precedence { Self::token_to_precedence(&self.current_token) }
    
    fn expect_peek(&mut self, tok: Token) -> bool {
        if mem::discriminant(&self.peek_token) == mem::discriminant(&tok) { self.next_token(); true } 
        else { self.errors.push(format!("Expected next token to be {:?}, got {:?} instead", tok, self.peek_token)); false }
    }
    fn expect_peek_is_ident(&mut self) -> bool {
        if let Token::Ident(_) = self.peek_token { self.next_token(); true }
        else { self.errors.push(format!("Expected next token to be an identifier, got {:?}", self.peek_token)); false }
    }
}
