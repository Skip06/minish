mod builtin;
mod exec;
mod lexer;
mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Word(String),
    Pipe,
    InputRedirect,
    OutputRedirect,
}

pub fn tokenize(input: &str) -> Vec<Tokens> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut quote_mode: Option<char> = None;

    for ch in input.chars() {
        match quote_mode {
            Some(active_quote) => {
                if ch == active_quote {
                    quote_mode = None;
                } else {
                    current_token.push(ch);
                }
            }
            None => {
                if ch == '"' || ch == '\'' {
                    quote_mode = Some(ch);
                } else if ch.is_whitespace() {
                    if !current_token.is_empty() {
                        tokens.push(Tokens::Word(current_token));
                        current_token = String::new();
                    }
                } else if ch == '|' || ch == '<' || ch == '>' {
                    if !current_token.is_empty() {
                        tokens.push(Tokens::Word(current_token));
                        current_token = String::new();
                    }

                    match ch {
                        '|' => tokens.push(Tokens::Pipe),
                        '<' => tokens.push(Tokens::InputRedirect),
                        '>' => tokens.push(Tokens::OutputRedirect),
                        _ => unreachable!(),
                    }
                } else {
                    current_token.push(ch);
                }
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(Tokens::Word(current_token));
    }

    tokens
}

fn main() {
    println!("minish");
    tokenize("echo 'hello world'");
}

#[test]
fn basic_split() {
    let tokens = tokenize("echo hello world");

    assert_eq!(
        tokens,
        vec![
            Tokens::Word("echo".to_string()),
            Tokens::Word("hello".to_string()),
            Tokens::Word("world".to_string()),
        ]
    );
}

#[test]
fn quoted_split() {
    let tokens = tokenize("echo \"hello world\"");

    assert_eq!(
        tokens,
        vec![
            Tokens::Word("echo".to_string()),
            Tokens::Word("hello world".to_string()),
        ]
    );
}

#[test]
fn single_quoted_split() {
    let tokens = tokenize("echo 'hello world'");

    assert_eq!(
        tokens,
        vec![
            Tokens::Word("echo".to_string()),
            Tokens::Word("hello world".to_string()),
        ]
    );
}

#[test]
fn pipe_tokens() {
    let tokens = tokenize("ls | wc");

    assert_eq!(
        tokens,
        vec![
            Tokens::Word("ls".to_string()),
            Tokens::Pipe,
            Tokens::Word("wc".to_string()),
        ]
    );
}

#[test]
fn redirection_tokens() {
    let tokens = tokenize("sort < names.txt > out.txt");

    assert_eq!(
        tokens,
        vec![
            Tokens::Word("sort".to_string()),
            Tokens::InputRedirect,
            Tokens::Word("names.txt".to_string()),
            Tokens::OutputRedirect,
            Tokens::Word("out.txt".to_string()),
        ]
    );
}
