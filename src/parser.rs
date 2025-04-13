use crate::{bo_error::BoError, common::{Keyword, Op, Token, TokenType, Value}};

pub struct Module{
    pub statements: Vec<Statement>,
}
#[derive(Debug)]
pub enum Statement{
    Expr(Expression),
    Assignment(Expression, Expression),// first expr must be assignable
    Let(String, Expression),
    Return(Option<Expression>),
    Loop(Expression), // expects block expression
    While(Expression, Expression), // expects conditional and block expressions
    None,
}
#[derive(Debug)]
pub enum Expression {
    Literal(Value),
    Unary(Op,Box<Expression>),
    Binary(Op,Box<Expression>,Box<Expression>), // operation, left, right
    If(Box<Expression>,Box<Expression>,Box<Expression>), // if, then, else
    Assignable(Box<Expression>),
    Block(Vec<Statement>, Option<Box<Expression>>),
    Array(Vec<Expression>),
    None,
}

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
    error: Option<BoError>,
    module: Module,
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
    fn set_err(&mut self, token: Token, message: &str) -> BoError{
        if let Some(e) = self.error.clone() {
            return e;
        }
        let e = BoError{
            message: message.to_string(),
            char_idx: token.idx,
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
            TokenType::Op(o)=>{self.advance(); o == op},
            _ => {false}
        }
    }
    fn match_keyword(&mut self, keyword: Keyword) -> bool{
        match self.peek().token_type{
            TokenType::Keyword(kw)=>{self.advance(); kw == keyword},
            _ => {false}
        }
    }
    fn expression(&mut self) -> Expression{
        self.primary()
    }
    fn primary(&mut self) -> Expression{
        let t = self.advance();
        if let Some(v) = t.is_value() {
            return Expression::Literal(v);
        }
        self.set_err(t, "Expected literal");
        Expression::None
    }
    fn block(&mut self)->Expression{
        //
    }
    fn top_statement(&mut self){
        let stmt = self.statement();
        self.module.statements.push(stmt);
        if !self.match_op(Op::Semicolon){
            self.set_err(self.previous(), "Expected semicolon");
        }
    }
    fn let_statement(&mut self)->Statement{
        // consume let
        self.advance();
        // get identifier
        let ident = self.advance();
        if let Some(s) = ident.is_ident() {
            // consume equals
            if !self.match_op(Op::Equal){
                self.set_err(self.peek(),"Expected equals sign");
                return Statement::None;
            }
            // parse expression
            let expr = self.expression();
            return Statement::Let(s, expr);
        }
        self.set_err(self.peek(),"Expected identifier");
        Statement::None
    }
    fn return_statement(&mut self)->Statement{
        // consume return keyword
        self.advance();
        let next = self.peek();
        if let Some(op) = next.is_op() {
            if op == Op::Semicolon {return Statement::Return(None);}
        }
        let expr = self.expression();
        Statement::Return(Some(expr))
    }
    fn loop_statement(&mut self)->Statement{
        // consume loop keyword
        self.advance();
    }
    fn statement(&mut self) -> Statement{
        let t = self.peek();
        // let return loop while
        if let Some(kw) = t.is_keyword() {
            match kw {
                Keyword::Let=>{self.let_statement()},
                Keyword::Return=>{self.return_statement()},
                _ => {panic!("at the disco");}
            }
        }
        else{
            let expr = self.expression();
            Statement::Expr(expr)
        }
    }
    pub fn parse(tokens: Vec<Token>) -> Result<Module, BoError>{
        let mut parser = Parser{
            tokens,
            current: 0,
            error: None,
            module: Module{statements: Vec::new()},
        };
        while !parser.at_eof() && !parser.has_error() {
            parser.top_statement();
        }
        if let Some(e) = parser.error {
            return Err(e);
        }
        Ok(parser.module)
    }
}