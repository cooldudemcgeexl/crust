use super::traits::ParseTokens;
use super::utils::ParserError;
use crate::semantics::SemanticsError;
use crate::tokens::Token;

#[derive(Debug)]
pub enum TypeMark {
    Integer,
    Float,
    String,
    Bool,
}

impl ParseTokens for TypeMark {
    fn parse(tokens: &mut super::utils::TokenQueue) -> Result<Self, ParserError> {
        match tokens.pop_front() {
            Some(Token::Integer) => Ok(TypeMark::Integer),
            Some(Token::Float) => Ok(TypeMark::Float),
            Some(Token::String) => Ok(TypeMark::String),
            Some(Token::Bool) => Ok(TypeMark::Bool),
            Some(token) => Err(ParserError::UnexpectedToken(
                String::from("TypeMark"),
                token,
            )),
            None => Err(ParserError::UnexpectedEOF(String::from("TypeMark"))),
        }
    }
}
#[derive(Debug)]
pub struct ArrayBound {
    pub number: Number,
}

impl ParseTokens for ArrayBound {
    fn parse(tokens: &mut super::utils::TokenQueue) -> Result<Self, ParserError> {
        Number::parse(tokens).map(|token| ArrayBound { number: token })
    }
}

#[derive(Debug)]
pub struct Number {
    pub literal_string: String,
}

impl ParseTokens for Number {
    fn parse(tokens: &mut super::utils::TokenQueue) -> Result<Self, ParserError> {
        match tokens.pop_front() {
            Some(Token::NumberLiteral(val)) => Ok(Number {
                literal_string: val,
            }),
            Some(token) => Err(ParserError::UnexpectedToken(
                String::from("NumberLiteral"),
                token,
            )),
            None => Err(ParserError::UnexpectedEOF(String::from("NumberLiteral"))),
        }
    }
}

impl TryFrom<Number> for usize {
    type Error = SemanticsError;
    fn try_from(value: Number) -> Result<Self, Self::Error> {
        let value = value.literal_string.replace("_", "");
        Ok(usize::from_str_radix(&value, 10)?)
    }
}
impl TryFrom<Number> for i64 {
    type Error = SemanticsError;
    fn try_from(value: Number) -> Result<Self, Self::Error> {
        let value = value.literal_string.replace("_", "");
        Ok(i64::from_str_radix(&value, 10)?)
    }
}
impl TryFrom<Number> for f64 {
    type Error = SemanticsError;
    fn try_from(value: Number) -> Result<Self, Self::Error> {
        let value = value.literal_string.replace("_", "");
        Ok(value.parse::<f64>()?)
    }
}
#[derive(Debug)]
pub struct StringNode {
    pub literal_string: String,
}

#[derive(Debug)]
pub struct Identifier {
    pub identifier_string: String,
}
