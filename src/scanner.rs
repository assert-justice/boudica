use crate::common::{Keyword, Op, Token, TokenType, Value};
use crate::bo_error::BoError;

#[derive(Default)]
pub struct Scanner{
    src: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    error: Option<BoError>,
}

impl Scanner{
    fn new() -> Self{
        Self{
            ..Default::default()
        }
    }
    fn at_eof(&self) -> bool{
        self.current >= self.src.len()
    }
    fn has_error(&self) -> bool{
        if let Some(_) = &self.error {
            return true;
        }
        false
    }
    fn peek(&self) -> char{
        self.src[self.current]
    }
    fn advance(&mut self) -> char{
        let c = self.peek();
        self.current += 1;
        c
    }
    fn is_next(&mut self, c: char) -> bool{
        if self.at_eof() {return false;}
        self.advance();
        if self.at_eof() {return false;}
        c == self.peek()
    }
    fn add_token(&mut self, token_type: TokenType){
        self.tokens.push(Token{token_type, idx: self.start});
    }
    fn add_literal(&mut self, val: Value){
        self.tokens.push(Token{token_type: TokenType::Literal(val), idx: self.start});
    }
    fn is_op(&mut self) -> bool{
        let c = self.peek();
        let op = match c {
            '+' => {
                if self.is_next('='){Op::PlusEqual}
                else {Op::Plus}
            },
            '-' => {
                if self.is_next('='){Op::MinusEqual}
                else {Op::Minus}
            },
            '*' => {
                if self.is_next('='){Op::StarEqual}
                else {Op::Star}
            },
            '/' => {
                if self.is_next('='){Op::FrontSlashEqual}
                else {Op::FrontSlash}
            },
            '%' => {
                if self.is_next('='){Op::PercentEqual}
                else {Op::Percent}
            },
            '!' => {
                if self.is_next('='){Op::BangEqual}
                else {Op::Bang}
            },
            '=' => {
                if self.is_next('='){Op::EqualEqual}
                else {Op::Equal}
            },
            '<' => {
                if self.is_next('='){Op::LessEqual}
                else if self.is_next('<'){Op::LessLess}
                else {Op::Less}
            },
            '>' => {
                if self.is_next('='){Op::GreaterEqual}
                else if self.is_next('>'){Op::GreaterGreater}
                else {Op::Greater}
            },
            '^' => {
                if self.is_next('='){Op::CarrotEqual}
                else {Op::Carrot}
            },
            '~' => {
                if self.is_next('='){Op::TildeEqual}
                else {Op::Tilde}
            },
            '.' => {
                if self.is_next('.'){Op::StopStop}
                else {Op::Stop}
            },
            '|' => {
                if self.is_next('|'){Op::PipePipe}
                else if self.is_next('='){Op::PipeEqual}
                else {Op::Pipe}
            },
            '&' => {
                if self.is_next('&'){Op::AndAnd}
                else if self.is_next('='){Op::AndEqual}
                else {Op::And}
            },
            ',' => Op::Comma,
            ':' => {
                if self.is_next(':'){Op::ColonColon}
                else {Op::Colon}
            },
            ';' => Op::Semicolon,
            '(' => Op::LeftParen,
            ')' => Op::RightParen,
            '[' => Op::LeftBracket,
            ']' => Op::RightBracket,
            '{' => Op::LeftBrace,
            '}' => Op::RightBrace,
            _ => {return false;}
        };
        self.add_token(TokenType::Op(op));
        self.advance();
        true
    }
    fn is_number(&mut self) -> bool{
        if !self.peek().is_numeric() {return false;}
        let mut is_float = false;
        // let mut hex_encoded = false;
        // TODO: handle hexadecimal, octal, and binary number literals
        // TODO: allow numbers to be split up by underscores
        // if self.is_match("0x"){}
        while !self.at_eof() && self.peek().is_numeric() {
            self.advance();
        }
        if !self.at_eof() && self.peek() == '.'{self.advance(); is_float = true;}
        while !self.at_eof() && self.peek().is_numeric() {
            self.advance();
        }
        let text = self.src[self.start..self.current].iter().collect::<String>();
        if is_float{
            self.add_literal(Value::Float(text.parse().expect("Failed to parse float")));
        }
        else{
            self.add_literal(Value::Int(text.parse().expect("Failed to parse integer")));
        }
        true
    }
    fn is_str(&mut self) -> bool{
        if self.peek() != '"' {return false;}
        self.advance();
        while !self.at_eof() && self.peek() != '"' {
            self.advance();
        }
        if self.at_eof(){
            self.set_err("Unclosed string!");
            return true;
        }
        self.advance();
        let lit = self.src[(self.start+1)..(self.current-1)].iter().collect::<String>();
        self.add_literal(Value::String(lit));
        true
    }
    fn add_keyword(&mut self, keyword: Keyword){
        self.add_token(TokenType::Keyword(keyword));
    }
    fn is_identifier(&mut self) -> bool{
        if self.peek() != '_' && !self.peek().is_alphabetic(){return false;}
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let lit = self.src[self.start..self.current].iter().collect::<String>();
        let lit = lit.as_str();
        match lit {
            "true"=>{self.add_literal(Value::Bool(true));},
            "false"=>{self.add_literal(Value::Bool(false));},
            "let"=>{self.add_keyword(Keyword::Let);}
            "pub"=>{self.add_keyword(Keyword::Pub);}
            "struct"=>{self.add_keyword(Keyword::Struct);}
            "import"=>{self.add_keyword(Keyword::Import);}
            "from"=>{self.add_keyword(Keyword::From);}
            "if"=>{self.add_keyword(Keyword::If);}
            "else"=>{self.add_keyword(Keyword::Else);}
            "while"=>{self.add_keyword(Keyword::While);}
            "loop"=>{self.add_keyword(Keyword::Loop);}
            "break"=>{self.add_keyword(Keyword::Break);}
            "continue"=>{self.add_keyword(Keyword::Continue);}
            "return"=>{self.add_keyword(Keyword::Return);}
            _ =>{self.add_token(TokenType::Identifier(lit.to_string()));}
        }
        // if lit == "true"{self.add_literal(Value::Bool(true));}
        // else if lit == "false"{self.add_literal(Value::Bool(false));}
        // else {self.add_token(TokenType::Identifier(lit));}
        true
    }
    // fn is_char(&mut self) -> bool{}
    fn set_err(&mut self, message: &str) -> BoError{
        let err = BoError{
            message: message.to_string(),
            char_idx: self.start,
        };
        self.error = Some(err.clone());
        err
    }
    pub fn scan(src: &str) -> Result<Vec<Token>, BoError>{
        let mut scanner = Scanner::new();
        scanner.src = src.chars().collect();
        while !scanner.at_eof() && !scanner.has_error() {
            scanner.start = scanner.current;
            if scanner.peek().is_ascii_whitespace(){scanner.advance();}
            else if scanner.is_op(){}
            else if scanner.is_number(){}
            else if scanner.is_str(){}
            else if scanner.is_identifier(){}
            else{
                return Err(scanner.set_err("Unexpected character!"));
            }
        }
        if let Some(e) = scanner.error{
            return Err(e);
        }
        Ok(scanner.tokens)
    }
}