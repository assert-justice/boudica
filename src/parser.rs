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
    Loop(Box<Statement>), // expects block expression
    While(Expression, Box<Statement>), // expects conditional and block expressions
    If(Expression, Box<Statement>, Box<Option<Statement>>), // if, then, else
    Block(Vec<Statement>),
    None,
}
#[derive(Debug)]
pub enum Expression {
    Literal(Value),
    Unary(Op,Box<Expression>),
    Binary(Op,Box<Expression>,Box<Expression>), // operation, left, right
    Assignable(Box<Expression>),
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
    fn peek(&self) -> Option<Token>{
        self.tokens.get(self.current).cloned()
    }
    fn previous(&self) -> Option<Token>{
        self.tokens.get(self.current-1).cloned()
    }
    fn advance(&mut self) -> Option<Token>{
        let t = self.peek();
        if let Some(t) = t{
            self.current += 1;
            Some(t)
        }
        else{
            None
        }
    }
    // fn match_type(&mut self, token_type: TokenType) -> bool{
    //     if token_type != self.peek().token_type {return false;}
    //     self.advance();
    //     true
    // }
    fn match_op(&mut self, op: Op) -> bool{
        if let Some(t) = self.peek() {
            match t.token_type{
                TokenType::Op(o)=>{self.advance(); o == op},
                _ => {false}
            }
        }
        else {
            false
        }
    }
    fn match_keyword(&mut self, keyword: Keyword) -> bool{
        if let Some(t) = self.peek() {
            match t.token_type{
                TokenType::Keyword(kw)=>{self.advance(); kw == keyword},
                _ => {false}
            }
        }
        else {
            false
        }
    }
    fn expression(&mut self) -> Expression{
        self.primary()
    }
    fn primary(&mut self) -> Expression{
        let t = self.advance().unwrap();
        if let Some(v) = t.is_value() {
            return Expression::Literal(v);
        }
        self.set_err(t, "Expected literal");
        Expression::None
    }
    fn block(&mut self)->Statement{
        self.advance(); //consume {
        let mut statements = Vec::new();
        while !self.match_op(Op::RightBrace) && !self.at_eof() {
            statements.push(self.statement());
        }
        Statement::Block(statements)
    }
    fn top_statement(&mut self){
        let stmt = self.statement();
        self.module.statements.push(stmt);
    }
    fn let_statement(&mut self)->Statement{
        // consume let
        self.advance();
        // get identifier
        let ident = self.advance().unwrap();
        if let Some(s) = ident.is_ident() {
            // consume equals
            if !self.match_op(Op::Equal){
                self.set_err(self.peek().unwrap(),"Expected equals sign");
                return Statement::None;
            }
            // parse expression
            let expr = self.expression();
            return Statement::Let(s, expr);
        }
        self.set_err(self.peek().unwrap(),"Expected identifier");
        Statement::None
    }
    fn return_statement(&mut self)->Statement{
        self.advance(); // consume return keyword
        let next = self.peek().unwrap();
        if let Some(op) = next.is_op() {
            if op == Op::Semicolon {return Statement::Return(None);}
        }
        let expr = self.expression();
        Statement::Return(Some(expr))
    }
    fn loop_statement(&mut self)->Statement{
        self.advance(); // consume loop keyword
        Statement::Loop(Box::new(self.block()))
    }
    fn while_statement(&mut self) -> Statement{
        self.advance(); // consume while keyword
        let condition = self.expression();
        let stmt = Box::new(self.block());
        Statement::While(condition, stmt)
    }
    fn if_statement(&mut self) -> Statement{
        self.advance(); // consume if keyword
        let condition = self.expression();
        let then = Box::new(self.block());
        let mut _else = if self.match_keyword(Keyword::Else){
            Some(self.block())
        }
        else{
            None
        };
        Statement::If(condition, then, Box::new(_else))

    }
    fn statement(&mut self) -> Statement{
        let t = self.peek().unwrap();
        // let return loop while
        let res = if let Some(kw) = t.is_keyword() {
            match kw {
                Keyword::Let=>{self.let_statement()},
                Keyword::Return=>{self.return_statement()},
                Keyword::Loop=>{self.loop_statement()},
                Keyword::While=>{self.while_statement()},
                Keyword::If=>{self.if_statement()},
                _ => {panic!("at the disco");}
            }
        }
        else{
            let expr = self.expression();
            Statement::Expr(expr)
        };
        if !self.match_op(Op::Semicolon){
            self.set_err(self.previous().unwrap(), "Expected semicolon");
            return Statement::None;
        }
        res
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