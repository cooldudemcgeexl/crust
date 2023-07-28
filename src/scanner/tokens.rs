use thiserror::Error;

#[derive(Debug)]
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
    pub fn from_char(symbol_char: char) -> Token {
        match symbol_char {
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
            _ => todo!(),
        }
    }

    pub fn from_compound_identifier(compound_chars: &str) -> Token {
        match compound_chars {
            ":=" => Token::Assignment,
            "==" => Token::EqualsComp,
            "!=" => Token::NotEquals,
            "<=" => Token::LessThanEq,
            ">=" => Token::GreaterThanEq,
            _ => todo!()
        }
    }

    pub fn num_literal_from_string(string: String) -> Token {
        Token::NumberLiteral(string.to_lowercase())
    }
    pub fn string_literal_from_string(string: String) -> Token {
        Token::StringLiteral(string.to_lowercase())
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
            _ => Token::Identifier(string)
        }
    }

}

#[derive(Error, Debug)]
enum TokenError {}

#[derive(Debug)]
pub enum BuildToken {
    /// Not building a token
    None,
    /// Building a multicharacter symbol (i.e. !=. ==, >=)
    CompoundSymbol(String),
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String)
}
