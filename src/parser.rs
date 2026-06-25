//a command can have arg, o/p file and i/p file => sort < name.txt > file.txt
// pipe line can have multiple commands => ls | wc
// like bfrust , parser is just a loop in which a match statement will be present. (maybe)


use crate::lexer::Tokens;

pub struct Cmd{
    pub argv: Vec<String>,
    pub stdin_from: Option<String>, 
    pub stdout_to: Option<String>,
}

pub struct Pipeline{
    pub commands: Vec<Cmd>,
}

pub fn parse(tokens: &[Tokens]) -> Result<Pipeline, String>{
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
                if current_cmd.argv.is_empty(){
                    return Err("expecting a argv before pipe ".to_string());
                }
                commands.push(current_cmd); // ls getting pushed 
                current_cmd = Cmd{          
                    argv: Vec::new(),
                    stdin_from: None,
                    stdout_to: None,
                };
                i += 1;
            }
            //for ls | wc => it should look like Pipeline{[Cmd{ls ...}, Cmd{wc ...}]} with Options as None. 
            
            Tokens::InputRedirect => {  // for the other one Pipeline{[Cmd{argv: vec!["sort"], "file.txt", "name.txt"}]}
                if i+1 >= tokens.len(){
                    return Err("there shoiuld be filename to take input from after < ".to_string());
                }
                if current_cmd.stdin_from.is_some() {  // something < something < something
                    return Err("multiple input redirections".to_string());
                }
                match &tokens[i + 1] {
                    Tokens::Word(filename) => {
                        current_cmd.stdin_from = Some(filename.clone());
                        
                    }
                    _ => return Err("expected filename after <".to_string()),
                }
                i += 2;
            }

            Tokens::OutputRedirect => {  
                if i+1 > tokens.len(){
                    return Err("there shoiuld be filename to take input from after < ".to_string());
                }
                if current_cmd.stdout_to.is_some() {  // something > something > something
                    return Err("multiple input redirections".to_string());
                }
                match &tokens[i + 1] {
                    Tokens::Word(filename) => {
                        current_cmd.stdout_to = Some(filename.clone());
                    }
                    _ => return Err("expected filename after >".to_string()),
                }
                i += 2;
            }
        }
       
    }
    
    // if current_cmd.argv.is_empty(){  // | ls => sees the | and reutrns empty commnad.
    //     return Err("expecting a command before pipe ".to_string());
    // } 

    // // what about ls | => err => pipeline has one command element but then next is nothing ie argv of current_Cmd is nothing.
    // if !commands.is_empty() &&  current_cmd.argv.is_empty(){
    //     return Err("expecting a command after pipe ".to_string());
    // }

    //this 2nd check could never execute . cause for samme ls |  it will enter same 1st check and reutrns 
    
    if current_cmd.argv.is_empty() {
        if !commands.is_empty() {
            return Err("expected command after '|'".to_string());
        } else {
            return Err("missing argv".to_string());
        }
    }
     // what bout ls | | wc ?? when it sees 2nd pipe it will push empty command to commands vec as per code 
     // just puttin a check before doing that => if argv is empty then err

    if !current_cmd.argv.is_empty(){  // cause pipes only push like ls | wc , ls is pushd but not wc also the 2nd exmaple is never pushed . 
        commands.push(current_cmd);
    }
    Ok(Pipeline{commands})
}