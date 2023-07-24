use std::io::stdin;

use anyhow::Result;

use crate::lexer::{Lexer, Token};

const PROMPT: &str = ">> ";

pub fn start() -> Result<()> {
    let scanner = stdin();

    loop {
        println!("{}", PROMPT);

        let mut buf = String::new();
        scanner.read_line(&mut buf)?;

        let mut l = Lexer::new(buf.clone());
        let mut tokens = Vec::new();

        loop {
            let token = l.next_token();
            if token == Token::EoF {
                break;
            }

            tokens.push(token);
        }

        println!("{:?}", tokens);
    }
}
