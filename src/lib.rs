use parser::Parser;

mod scanner;
mod parser;
mod common;
mod bo_error;

pub fn temp(){
    let src = "return; \"hello\";";
    let res = scanner::Scanner::scan(src);
    let tokens = match res {
        Ok(ts)=>{ts},
        Err(e)=>{e.pretty_print(src); return;}
    };
    // for t in &tokens{
    //     println!("{:?}", t);
    // }
    let res = Parser::parse(tokens);
    let module = match res {
        Ok(m)=>{m},
        Err(e)=>{e.pretty_print(src);return;}
    };
    for s in &module.statements{
        println!("{:?}", s);
    }
}
