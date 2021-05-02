// Copyright (C) 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

mod keywords;
mod signs;

pub use keywords::*;
pub use signs::*;

pub enum Token {
    Identifier(String),
    String(String),
    Number(i64),
    Float(f64),
    Keyword(Keywords),
    Sign(Signs),
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Token::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Token::Float(_))
    }

    pub fn get_keyword(&self) -> Option<Keywords> {
        match self {
            Token::Keyword(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_sign(&self) -> Option<Signs> {
        match self {
            Token::Sign(value) => Some(value.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_identifier() {
        assert!(Token::Identifier(String::new()).is_identifier());
    }

    #[test]
    fn test_is_string() {
        assert!(Token::String(String::new()).is_string());
    }

    #[test]
    fn test_is_number() {
        assert!(Token::Number(123).is_number());
    }

    #[test]
    fn test_is_float() {
        assert!(Token::Float(123.123).is_float());
    }

    #[test]
    fn test_get_keyword() {
        assert_eq!(
            Token::Keyword(Keywords::Fn).get_keyword().unwrap(),
            Keywords::Fn
        );
    }

    #[test]
    fn test_get_sign() {
        assert_eq!(
            Token::Sign(Signs::Semicolon).get_sign().unwrap(),
            Signs::Semicolon
        );
    }
}
