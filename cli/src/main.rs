use std::env;

use vitefest_cli::{
    tokenizer::{token::Token, tokenize}, 
    executer::execute
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let token_args: Vec<Token> = tokenize(args);

    execute(token_args);
}