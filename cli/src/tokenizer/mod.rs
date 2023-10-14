pub mod token;

use self::token::Token;

pub fn tokenize(args: Vec<String>) -> Vec<Token> {
    args.iter().map(|arg| Token::new(arg))
               .collect::<Vec<Token>>()
}

pub fn peek_next(args: &Vec<Token>, current: &Token) -> Token {
    let current_index = args.iter().position(|arg| arg == current);

    let next_index: Option<usize> = match current_index {
        Some(i) => {
            if i == args.len() - 1 {
                None
            } else {
              Some(i + 1)
            }
        },
        None => None
    };

    match next_index {
        Some(i) => {
            args[i].clone()
        },
        None => {
            Token::new("")
        }
    }
}