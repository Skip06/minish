mod builtin;
mod exec;
mod lexer;
mod parser;



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
