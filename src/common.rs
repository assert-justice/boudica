use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Op{
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    FrontSlash,
    FrontSlashEqual,
    Percent,
    PercentEqual,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    LessLess,
    Greater,
    GreaterEqual,
    GreaterGreater,
    Carrot,
    CarrotEqual,
    Tilde,
    TildeEqual,
    Colon,
    Semicolon,
    Stop,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType{
    Op(Op),
    Literal(Value),
    Identifier(String),
}
#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub idx: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    // Char(char),
    String(String),
    Array(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function(usize), // block of bytecode to jump to
}