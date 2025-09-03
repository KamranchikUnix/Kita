use super::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i64),
    Boolean(bool),
    Prefix { op: Token, right: Box<Expression> },
    Infix { op: Token, left: Box<Expression>, right: Box<Expression> },
    If { condition: Box<Expression>, consequence: BlockStatement, alternative: Option<BlockStatement> },
    FunctionLiteral { params: Vec<String>, body: BlockStatement },
    Call { function: Box<Expression>, arguments: Vec<Expression> },
}

pub type Program = Vec<Statement>;
pub type BlockStatement = Vec<Statement>;
