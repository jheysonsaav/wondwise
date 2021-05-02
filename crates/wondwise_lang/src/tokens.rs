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
