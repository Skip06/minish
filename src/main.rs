mod builtin;
mod exec;
mod lexer;
mod parser;

use lexer::tokenize;
use parser::parse;
use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("minish > ");
        let mut input = String::new();

        io::stdout().flush().unwrap(); // this forces minish> to appear immedietly like no need to press enter.
        std::io::stdin().read_line(&mut input).expect("could not take input");

        let line  = input.trim();
        let tokens = tokenize(&line);
        let parsed = tokens.as_ref().map(|t| parse(t)).unwrap();

        
        match parsed{
            Ok(pipeline) => {
                println!("{:?}", pipeline);
                let _ = exec::exec(pipeline);
            },
            Err(err) => println!("{:?}", err) // for some reason print!() does not work println!() works maybe ln adds newline 
        }
        if line == "exit"{
            break;
        }
    }
}
