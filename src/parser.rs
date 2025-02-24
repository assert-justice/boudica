use std::thread::current;

use crate::{bo_error::BoError, common::{Op, Token, TokenType, Value}};

pub struct Ast{
    statements: Vec<Statement>,
}

pub enum Statement{
    Expr(Expression),
    Assignment(Expression),
}
pub enum Expression {
    Literal(Value),
    Unary(Op,Box<Expression>),
    Binary(Op,Box<Expression>,Box<Expression>), // operation, left, right
    If(Box<Expression>,Box<Expression>,Box<Expression>), // if, then, else
    Assignable(Box<Expression>),
    Block(Vec<Statement>),
}

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
    error: Option<BoError>,
    ast: Ast,
}

impl Parser {
    fn at_eof(&self) -> bool{
        self.current >= self.tokens.len()
    }
    fn has_error(&self) -> bool{
        if let Some(_) = self.error{
            return true;
        }
        false
    }
    fn set_err(&mut self, message: &str) -> BoError{
        let e = BoError{
            message: message.to_string(),
            char_idx: self.peek().idx,
        };
        self.error = Some(e.clone());
        e
    }
    fn peek(&self) -> Token{
        self.tokens[self.current].clone()
    }
    fn previous(&self) -> Token{
        self.tokens[self.current-1].clone()
    }
    fn advance(&mut self) -> Token{
        let t = self.peek();
        self.current += 1;
        t
    }
    fn match_type(&mut self, token_type: TokenType) -> bool{
        if token_type != self.peek().token_type {return false;}
        self.advance();
        true
    }
    fn match_op(&mut self, op: Op) -> bool{
        match self.peek().token_type{
            TokenType::Op(o)=>{o == op},
            _ => {false}
        }
    }
    fn top_statement(&mut self){
        let t = self.peek();
    }
    pub fn parse(tokens: Vec<Token>) -> Result<Ast, BoError>{
        let mut parser = Parser{
            tokens,
            current: 0,
            error: None,
            ast: Ast{statements: Vec::new()},
        };
        while !parser.at_eof() && !parser.has_error() {
            //
        }
        if let Some(e) = parser.error {
            return Err(e);
        }
        Ok(parser.ast)
    }
}