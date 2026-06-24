mod builtin;
mod exec;
mod lexer;
mod parser;

fn main() {
    println!("minish");
    tokenize("echo 'hello world'");
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut quote_mode: Option<char> = None; 
    
    for ch in input.chars(){
        match quote_mode {
            Some(active_quote) =>{
                if ch == active_quote{
                    quote_mode = None;
                }
                current_token.push(ch);
            }
            None => {
                if ch == '"' ||ch == '/'{
                    quote_mode = Some(ch);
                }
                else if ch.is_whitespace(){
                    if !current_token.is_empty(){
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                    
                }
                else{
                    current_token.push(ch);
                }
            }
        }
    }
    tokens
}

#[test]
fn basic_split() {
    let tokens = tokenize("echo hello world");

    assert_eq!(tokens, vec!["echo", "hello", "world"]);
}

#[test]
fn quoted_split() {
    let tokens = tokenize("echo \"hello world\"");

    assert_eq!(tokens, vec!["echo", "hello world"]);
}

#[test]
fn single_quoted_split() {
    let tokens = tokenize("echo 'hello world'");

    assert_eq!(tokens, vec!["echo", "hello world"]);
}
