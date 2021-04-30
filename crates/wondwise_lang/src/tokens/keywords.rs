// Copyright (C) 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

use std::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Keywords {
    Let,
    Const,

    Fn,
    Return,

    True,
    False,

    For,
    In,
    Continue,
    Break,
    While,

    If,
    Else,
    Switch,

    Import,
    From,
    Export,
}

impl Keywords {
    #[allow(clippy::result_unit_err)]
    pub fn from(value: &str) -> Result<Keywords, ()> {
        match value {
            "let" => Ok(Keywords::Let),
            "const" => Ok(Keywords::Const),

            "fn" => Ok(Keywords::Fn),
            "return" => Ok(Keywords::Return),

            "true" => Ok(Keywords::True),
            "false" => Ok(Keywords::False),

            "for" => Ok(Keywords::For),
            "in" => Ok(Keywords::In),
            "continue" => Ok(Keywords::Continue),
            "break" => Ok(Keywords::Break),
            "while" => Ok(Keywords::While),

            "if" => Ok(Keywords::If),
            "else" => Ok(Keywords::Else),
            "switch" => Ok(Keywords::Switch),

            "import" => Ok(Keywords::Import),
            "from" => Ok(Keywords::From),
            "export" => Ok(Keywords::Export),

            _ => Err(()),
        }
    }
}

impl fmt::Display for Keywords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keywords::Let => write!(f, "let"),
            Keywords::Const => write!(f, "const"),

            Keywords::Fn => write!(f, "fn"),
            Keywords::Return => write!(f, "return"),

            Keywords::True => write!(f, "true"),
            Keywords::False => write!(f, "false"),

            Keywords::For => write!(f, "for"),
            Keywords::In => write!(f, "in"),
            Keywords::Continue => write!(f, "continue"),
            Keywords::Break => write!(f, "break"),
            Keywords::While => write!(f, "while"),

            Keywords::If => write!(f, "if"),
            Keywords::Else => write!(f, "else"),
            Keywords::Switch => write!(f, "switch"),

            Keywords::Import => write!(f, "import"),
            Keywords::From => write!(f, "from"),
            Keywords::Export => write!(f, "export"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_from() {
        assert_eq!(Keywords::from("let").unwrap(), Keywords::Let);
        assert_eq!(Keywords::from("const").unwrap(), Keywords::Const);

        assert_eq!(Keywords::from("fn").unwrap(), Keywords::Fn);
        assert_eq!(Keywords::from("return").unwrap(), Keywords::Return);

        assert_eq!(Keywords::from("true").unwrap(), Keywords::True);
        assert_eq!(Keywords::from("false").unwrap(), Keywords::False);

        assert_eq!(Keywords::from("for").unwrap(), Keywords::For);
        assert_eq!(Keywords::from("in").unwrap(), Keywords::In);
        assert_eq!(Keywords::from("continue").unwrap(), Keywords::Continue);
        assert_eq!(Keywords::from("break").unwrap(), Keywords::Break);
        assert_eq!(Keywords::from("while").unwrap(), Keywords::While);

        assert_eq!(Keywords::from("if").unwrap(), Keywords::If);
        assert_eq!(Keywords::from("else").unwrap(), Keywords::Else);
        assert_eq!(Keywords::from("switch").unwrap(), Keywords::Switch);

        assert_eq!(Keywords::from("import").unwrap(), Keywords::Import);
        assert_eq!(Keywords::from("from").unwrap(), Keywords::From);
        assert_eq!(Keywords::from("export").unwrap(), Keywords::Export);
    }

    #[test]
    fn test_keyword_to_string() {
        assert_eq!(Keywords::Let.to_string(), "let");
        assert_eq!(Keywords::Const.to_string(), "const");

        assert_eq!(Keywords::Fn.to_string(), "fn");
        assert_eq!(Keywords::Return.to_string(), "return");

        assert_eq!(Keywords::True.to_string(), "true");
        assert_eq!(Keywords::False.to_string(), "false");

        assert_eq!(Keywords::For.to_string(), "for");
        assert_eq!(Keywords::In.to_string(), "in");
        assert_eq!(Keywords::Continue.to_string(), "continue");
        assert_eq!(Keywords::Break.to_string(), "break");
        assert_eq!(Keywords::While.to_string(), "while");

        assert_eq!(Keywords::If.to_string(), "if");
        assert_eq!(Keywords::Else.to_string(), "else");
        assert_eq!(Keywords::Switch.to_string(), "switch");

        assert_eq!(Keywords::Import.to_string(), "import");
        assert_eq!(Keywords::From.to_string(), "from");
        assert_eq!(Keywords::Export.to_string(), "export");
    }
}
