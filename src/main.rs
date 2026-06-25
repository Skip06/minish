mod builtin;
mod exec;
mod lexer;
mod parser;

use lexer::tokenize;
use parser::parse;



fn main() {
    println!("minish");
    tokenize("echo 'hello world'");
}



