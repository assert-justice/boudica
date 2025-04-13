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
    ColonColon,
    Semicolon,
    Stop,
    StopStop,
    Pipe,
    PipePipe,
    PipeEqual,
    And,
    AndAnd,
    AndEqual,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Pub,
    Struct,
    Import,
    From,
    If,
    Else,
    While,
    Loop,
    Break,
    Continue,
    Return,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType{
    Op(Op),
    Keyword(Keyword),
    Literal(Value),
    Identifier(String),
}
#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub idx: usize,
}
impl Token {
    pub fn is_keyword(&self) -> Option<Keyword>{
        match self.token_type.clone() {
            TokenType::Keyword(kw)=>Some(kw),
            _ => None,
        }
    }
    pub fn is_ident(&self) -> Option<String>{
        match self.token_type.clone() {
            TokenType::Identifier(s)=>Some(s),
            _ => None,
        }
    }
    pub fn is_value(&self) -> Option<Value>{
        match self.token_type.clone() {
            TokenType::Literal(v)=>Some(v),
            _ => None,
        }
    }
    pub fn is_op(&self) -> Option<Op>{
        match self.token_type.clone() {
            TokenType::Op(op)=>Some(op),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function(usize), // block of bytecode to jump to
}