use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
/// Terminals from the EBNF Grammar Provided
pub enum Token {
    // Keywords
    Program,
    Is,
    Begin,
    End,
    Global,
    Procedure,
    Variable,
    For,
    Not,
    If,
    Then,
    Else,
    Return,
    True,
    False,

    // Types
    Integer,
    Float,
    String,
    Bool,

    // Symbols
    Plus,
    Minus,
    Mult,
    Div,
    GreaterThan,
    LessThan,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Amp,
    Pipe,
    Colon,
    Period,
    Semicolon,
    Comma,

    // Symbol Pairs
    GreaterThanEq,
    LessThanEq,
    EqualsComp,
    NotEquals,
    Assignment,

    // Identifiers
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),

    // Special :)
    EOF,
}

impl Token {
    /// Returns the token for single character tokens
    pub fn from_char(symbol_char: char) -> Result<Token, TokenError> {
        let matched_token = match symbol_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Mult,
            '/' => Token::Div,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '&' => Token::Amp,
            '|' => Token::Pipe,
            ':' => Token::Colon,
            '.' => Token::Period,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            _ => return Err(TokenError::SingleTokenError(symbol_char)),
        };
        Ok(matched_token)
    }

    pub fn from_compound_identifier(compound_chars: &str) -> Result<Token, TokenError> {
        let compound_token = match compound_chars {
            ":=" => Token::Assignment,
            "==" => Token::EqualsComp,
            "!=" => Token::NotEquals,
            "<=" => Token::LessThanEq,
            ">=" => Token::GreaterThanEq,
            _ => return Err(TokenError::CompoundTokenError(String::from(compound_chars))),
        };
        Ok(compound_token)
    }

    pub fn num_literal_from_string(string: String) -> Token {
        Token::NumberLiteral(string.to_lowercase())
    }
    pub fn string_literal_from_string(string: String) -> Token {
        Token::StringLiteral(string)
    }

    pub fn from_string(string: String) -> Token {
        match string.to_lowercase().as_str() {
            "program" => Token::Program,
            "is" => Token::Is,
            "begin" => Token::Begin,
            "end" => Token::End,
            "global" => Token::Global,
            "procedure" => Token::Procedure,
            "variable" => Token::Variable,
            "for" => Token::For,
            "not" => Token::Not,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "integer" => Token::Integer,
            "float" => Token::Float,
            "string" => Token::String,
            "bool" => Token::Bool,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(string.to_lowercase()),
        }
    }
}

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Unrecognized token {0}")]
    SingleTokenError(char),
    #[error("Unreognized compount token {0}")]
    CompoundTokenError(String),
}

#[derive(Debug)]
pub enum BuildToken {
    /// Not building a token
    None,
    /// Building a multicharacter symbol (i.e. !=. ==, >=)
    CompoundSymbol(String),
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
}
