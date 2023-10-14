mod react_extension_creator;

use crate::tokenizer::{
        token::{Token, TokenType}, 
        peek_next
    };

use self::react_extension_creator::{ 
    create_react_normal_extension, 
    create_react_lite_extension
};

const HELP: &'static str = include_str!("../../resources/help.txt");
const POST_CREATE: &'static str = include_str!("../../resources/post_create.txt");

/**
 * execute command
 */ 
pub fn execute(args: Vec<Token>) {
    let post_create: String = POST_CREATE.to_string();
    let help: String = HELP.to_string().replace("${version}", env!("CARGO_PKG_VERSION"));

    // analyze first parameter
    match &args[1].token_type {
        TokenType::Help => {
            if let TokenType::End = peek_next(&args, &args[1]).token_type {
                println!("{}", help);
            } else {
                println!("Syntax Error! \nUsage: \n  vitefest-cli help");
            }
        },
        TokenType::Version => {
            if let TokenType::End = peek_next(&args, &args[1]).token_type {
                println!(
                    "v{}", 
                    env!("CARGO_PKG_VERSION")
                );
            } else {
                println!("Syntax Error! \nUsage: \n  vitefest-cli version");
            }
        },
        TokenType::Create => {
            match peek_next(&args, &args[1]).token_type {
                TokenType::Normal => {
                    match peek_next(&args, &args[2]).token_type {
                        TokenType::End => {
                            create_react_normal_extension("vite-react-crx", Some("A Vite + React Chrome Extension."));
                            println!("{}", post_create.replace("${project_info}", "vite-react-crx"));
                        },
                        TokenType::Name => {
                            if let TokenType::Others = peek_next(&args, &args[3]).token_type {
                                create_react_normal_extension(
                        (&args[4]).text.as_str(), 
                         Some("A Vite + React Chrome Extension.")
                                );
                                println!("{}", post_create.replace("${project_info}", (&args[4]).text.as_str()));
                            } else {
                                println!("Syntax Error! \nUsage: \n  vitefest-cli create normal [--name <app-name>]")
                            }
                        },
                        _ => println!("Syntax Error! \nUsage: \n  vitefest-cli create normal [--name <app-name>]")
                    }

                    
                },
                TokenType::Lite => {
                    match peek_next(&args, &args[2]).token_type {
                        TokenType::End => {
                            create_react_lite_extension("vite-react-crx", Some("A Vite + React Chrome Extension."));
                            println!("{}", post_create.replace("${project_info}", "vite-react-crx"));
                        },
                        TokenType::Name => {
                            if let TokenType::Others = peek_next(&args, &args[3]).token_type {
                                create_react_lite_extension(
                        (&args[4]).text.as_str(), 
                         Some("A Vite + React Chrome Extension.")
                                );
                                println!("{}", post_create.replace("${project_info}", (&args[4]).text.as_str()));
                            } else {
                                println!("Syntax Error! \nUsage: \n  vitefest-cli create lite [--name <app-name>]")
                            }
                        },
                        _ => println!("Syntax Error! \nUsage: \n  vitefest-cli create lite [--name <app-name>]")
                    }
                },
                TokenType::End  => {
                    create_react_normal_extension("vite-react-crx", Some("A Vite + React Chrome Extension."));
                    println!("{}", post_create.replace("${project_info}", "vite-react-crx"));
                },
                _ => println!("Syntax Error! \nUsage: \n  vitefest-cli create normal\n  vitefest-cli create lite")
            };
        },
        _ => {
            println!("Syntax Error! \n{}", help);
        }
    };
}