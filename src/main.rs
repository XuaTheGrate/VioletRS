#[macro_use]
extern crate lazy_static;

mod lexer;

fn main() {
    let mut lex = lexer::Lexer::from_file("./samples/main.vi");
    let tokens = lex.analyze();
    println!("{:?}", tokens);
}
