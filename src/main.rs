#![allow(clippy::needless_return)]

mod ast;
mod lexer;
mod parser;
mod repl;

fn main() {
    println!("Welcome to PEPEGA lang, you can start writing commands");
    repl::start().unwrap();
}
