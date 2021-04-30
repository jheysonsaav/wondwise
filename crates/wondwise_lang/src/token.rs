// Copyright (C) 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    // General
    Identifier(String),
    Str(String),
    Num(f64),

    True,
    False,

    // Keywords
    Let,
    Const,

    Fn,
    Return,

    If,
    Else,

    Import,
    Export,

    // Signs
    Dot,
    Comma,
    Colon,
    Semicolon,

    Equal,
    DoubleEqual,
    EqualGreater,

    Not,
    NotEqual,

    Plus,
    PlusEqual,

    Minus,
    MinusEqual,

    Star,
    StarEqual,
    DoubleStar,
    DoubleStarEqual,

    Slash,
    SlashEqual,

    Percent,
    PercentEqual,

    Less,
    LessEqual,

    Greater,
    GreaterEqual,

    LeftParentheses,
    RightParentheses,

    LeftBrace,
    RightBrace,

    LeftBracket,
    RightBracket,

    DoubleVBar,
    DoubleAmper,

    // Other
    EndOfLine,
    EndOfFile,
}

#[allow(dead_code)] // TODO: delete this code line with use c
impl Token {
    /// Convert the token to a string.
    pub fn to_string(&self) -> String {
        match self {
            // General
            Self::Identifier(_) => String::from("Identifier"),
            Self::Str(_) => String::from("String"),
            Self::Num(_) => String::from("Number"),

            Self::True => String::from("true"),
            Self::False => String::from("false"),

            // Keywords
            Self::Let => String::from("let"),
            Self::Const => String::from("const"),

            Self::Fn => String::from("fn"),
            Self::Return => String::from("return"),

            Self::If => String::from("if"),
            Self::Else => String::from("else"),

            Self::Import => String::from("import"),
            Self::Export => String::from("export"),

            // Signs
            Self::Dot => String::from("."),
            Self::Comma => String::from(","),
            Self::Colon => String::from(":"),
            Self::Semicolon => String::from(";"),

            Self::Equal => String::from("="),
            Self::DoubleEqual => String::from("=="),
            Self::EqualGreater => String::from("=>"),

            Self::Not => String::from("!"),
            Self::NotEqual => String::from("!="),

            Self::Plus => String::from("+"),
            Self::PlusEqual => String::from("+="),

            Self::Minus => String::from("-"),
            Self::MinusEqual => String::from("-="),

            Self::Star => String::from("*"),
            Self::StarEqual => String::from("*="),
            Self::DoubleStar => String::from("**"),
            Self::DoubleStarEqual => String::from("**="),

            Self::Slash => String::from("/"),
            Self::SlashEqual => String::from("/="),

            Self::Percent => String::from("%"),
            Self::PercentEqual => String::from("%="),

            Self::Less => String::from("<"),
            Self::LessEqual => String::from("<="),

            Self::Greater => String::from(">"),
            Self::GreaterEqual => String::from(">="),

            Self::LeftParentheses => String::from("("),
            Self::RightParentheses => String::from(")"),

            Self::LeftBrace => String::from("{"),
            Self::RightBrace => String::from("}"),

            Self::LeftBracket => String::from("["),
            Self::RightBracket => String::from("]"),

            Self::DoubleVBar => String::from("||"),
            Self::DoubleAmper => String::from("&&"),

            // Other
            Self::EndOfLine => String::from("\n"),
            Self::EndOfFile => String::from("<<EOF>>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_string() {
        assert_eq!(Token::True.to_string(), "true");
        assert_eq!(Token::False.to_string(), "false");
        assert_eq!(Token::Let.to_string(), "let");
        assert_eq!(Token::Const.to_string(), "const");
        assert_eq!(Token::Fn.to_string(), "fn");
        assert_eq!(Token::Return.to_string(), "return");
        assert_eq!(Token::If.to_string(), "if");
        assert_eq!(Token::Else.to_string(), "else");
        assert_eq!(Token::Dot.to_string(), ".");
        assert_eq!(Token::Comma.to_string(), ",");
        assert_eq!(Token::EndOfLine.to_string(), "\n");
        assert_eq!(Token::EndOfFile.to_string(), "<<EOF>>");
    }
}
