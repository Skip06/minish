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
            Some(active_quote) => { // if inside quote then treat whitespace as normal text
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

    tokens // e.g => [Word("ls"),Pipe,Word("wc")] this will be the i/p to the parser
}