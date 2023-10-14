#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Help,

    // sub-options of Create: Normal, Lite
    Create,  
    Normal,
    Lite,

    Name,
    Run,
    Version,
    End,
    Others
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType
}

fn judge_token_type(text: &str) -> TokenType {
    match text {
        "help" | "-h"          => TokenType::Help,

        // sub-options of Create: Normal, Lite
        "create" | "init"              => TokenType::Create,
        "normal"                       => TokenType::Normal,
        "lite"                         => TokenType::Lite,

        "--name"                       => TokenType::Name,
        "run"                          => TokenType::Run,
        "version" | "--version" | "-v" => TokenType::Version,
        ""                             => TokenType::End,
        _                              => TokenType::Others
    }
}

impl Token {
    pub fn new(text: &str) -> Self {
        let token_type: TokenType = judge_token_type(text);
        
        Self {
            text: String::from(text),
            token_type
        }
    }
}