//a command can have arg, o/p file and i/p file => sort < name.txt > file.txt
// pipe line can have multiple commands => ls | wc
// like bfrust , parser is just a loop in which a match statement will be present. (maybe)

use std::thread::current;

use crate::lexer::Tokens;

pub struct Cmd{
    pub argv: Vec<String>,
    pub stdin_from: Option<String>, 
    pub stdout_to: Option<String>,
}

pub struct Pipeline{
    pub commands: Vec<Cmd>,
}

pub fn parser(tokens: &[Tokens]) -> Pipeline{
    let mut commands = Vec::new(); //empty pipeline
    let mut current_cmd = Cmd{   
        argv: Vec::new(),
        stdin_from: None,
        stdout_to: None,
    };
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Tokens::Word(word) => {    // just taking 2 examples sort < name.txt > file.txt and ls | wc
                current_cmd.argv.push(word.clone());
                i += 1;
            },
            Tokens::Pipe => {
                commands.push(current_cmd);
                current_cmd = Cmd{
                    argv: Vec::new(),
                    stdin_from: None,
                    stdout_to: None,
                };
                i += 1;
            }
            //for ls | wc => it should look like Pipeline{Cmd{...}, Cmd{...}} with Options as None. 
            
            Tokens::InputRedirect => {  // for the other one Pipeline{Cmd{argv: vec!["sort"], "file.txt", "name.txt"}}
                if i+1 > tokens.len(){
                    panic!("there shoiuld be filename to take input from after < ");
                }
                match &tokens[i + 1] {
                    Tokens::Word(filename) => {
                        current_cmd.stdin_from = Some(filename.clone());
                    }
                    _ => panic!("expected filename after <"),
                }
                i += 2;
            }

            Tokens::OutputRedirect => {  
                if i+1 > tokens.len(){
                    panic!("there shoiuld be filename to take input from after < ");
                }
                match &tokens[i + 1] {
                    Tokens::Word(filename) => {
                        current_cmd.stdout_to = Some(filename.clone());
                    }
                    _ => panic!("expected filename after <"),
                }
                i += 2;
            }
        }
       
    }
    if !current_cmd.argv.is_empty(){
        commands.push(current_cmd);
    }
    Pipeline{commands}
}