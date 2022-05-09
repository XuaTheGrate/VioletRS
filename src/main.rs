use std::error::Error;

#[macro_use]
extern crate lazy_static;

pub mod lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lex = lexer::Lexer::from_file("./samples/main.vi")?;
    let tokens = lex.analyze();
    println!("{:?}", tokens);
    Ok(())
}
