// Copyright (C) 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

#[allow(dead_code)] // TODO: remove this when no longer needed
#[derive(Debug, PartialEq)]
pub enum Signs {
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

    EndOfLine,
    EndOfFile,
}

#[allow(dead_code)] // TODO: remove this when no longer needed
impl Signs {
    pub fn from(value: &str) -> Result<Signs, ()> {
        match value {
            "." => Ok(Signs::Dot),
            "," => Ok(Signs::Comma),

            ":" => Ok(Signs::Colon),
            ";" => Ok(Signs::Semicolon),

            "=" => Ok(Signs::Equal),
            "==" => Ok(Signs::DoubleEqual),
            "=>" => Ok(Signs::EqualGreater),

            "!" => Ok(Signs::Not),
            "!=" => Ok(Signs::NotEqual),

            "+" => Ok(Signs::Plus),
            "+=" => Ok(Signs::PlusEqual),

            "-" => Ok(Signs::Minus),
            "-=" => Ok(Signs::MinusEqual),

            "<" => Ok(Signs::Less),
            "<=" => Ok(Signs::LessEqual),

            ">" => Ok(Signs::Greater),
            ">=" => Ok(Signs::GreaterEqual),

            "(" => Ok(Signs::LeftParentheses),
            ")" => Ok(Signs::RightParentheses),

            "{" => Ok(Signs::LeftBrace),
            "}" => Ok(Signs::RightBrace),

            "[" => Ok(Signs::LeftBracket),
            "]" => Ok(Signs::RightBracket),

            "||" => Ok(Signs::DoubleVBar),
            "&&" => Ok(Signs::DoubleAmper),

            "\n" => Ok(Signs::EndOfLine),
            "<<EOF>>" => Ok(Signs::EndOfFile),

            _ => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Signs::Dot => String::from("."),
            Signs::Comma => String::from(","),

            Signs::Colon => String::from(":"),
            Signs::Semicolon => String::from(";"),

            Signs::Equal => String::from("="),
            Signs::DoubleEqual => String::from("=="),
            Signs::EqualGreater => String::from("=>"),

            Signs::Not => String::from("!"),
            Signs::NotEqual => String::from("!="),

            Signs::Plus => String::from("+"),
            Signs::PlusEqual => String::from("+="),

            Signs::Minus => String::from("-"),
            Signs::MinusEqual => String::from("-="),

            Signs::Less => String::from("<"),
            Signs::LessEqual => String::from("<="),

            Signs::Greater => String::from(">"),
            Signs::GreaterEqual => String::from(">="),

            Signs::LeftParentheses => String::from("("),
            Signs::RightParentheses => String::from(")"),

            Signs::LeftBrace => String::from("{"),
            Signs::RightBrace => String::from("}"),

            Signs::LeftBracket => String::from("["),
            Signs::RightBracket => String::from("]"),

            Signs::DoubleVBar => String::from("||"),
            Signs::DoubleAmper => String::from("&&"),

            Signs::EndOfLine => String::from("\n"),
            Signs::EndOfFile => String::from("<<EOF>>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signs_from() {
        assert_eq!(Signs::from(".").unwrap(), Signs::Dot);
        assert_eq!(Signs::from(",").unwrap(), Signs::Comma);

        assert_eq!(Signs::from(":").unwrap(), Signs::Colon);
        assert_eq!(Signs::from(";").unwrap(), Signs::Semicolon);

        assert_eq!(Signs::from("=").unwrap(), Signs::Equal);
        assert_eq!(Signs::from("==").unwrap(), Signs::DoubleEqual);
        assert_eq!(Signs::from("=>").unwrap(), Signs::EqualGreater);

        assert_eq!(Signs::from("!").unwrap(), Signs::Not);
        assert_eq!(Signs::from("!=").unwrap(), Signs::NotEqual);

        assert_eq!(Signs::from("+").unwrap(), Signs::Plus);
        assert_eq!(Signs::from("+=").unwrap(), Signs::PlusEqual);

        assert_eq!(Signs::from("-").unwrap(), Signs::Minus);
        assert_eq!(Signs::from("-=").unwrap(), Signs::MinusEqual);

        assert_eq!(Signs::from("<").unwrap(), Signs::Less);
        assert_eq!(Signs::from("<=").unwrap(), Signs::LessEqual);

        assert_eq!(Signs::from(">").unwrap(), Signs::Greater);
        assert_eq!(Signs::from(">=").unwrap(), Signs::GreaterEqual);

        assert_eq!(Signs::from("(").unwrap(), Signs::LeftParentheses);
        assert_eq!(Signs::from(")").unwrap(), Signs::RightParentheses);

        assert_eq!(Signs::from("{").unwrap(), Signs::LeftBrace);
        assert_eq!(Signs::from("}").unwrap(), Signs::RightBrace);

        assert_eq!(Signs::from("[").unwrap(), Signs::LeftBracket);
        assert_eq!(Signs::from("]").unwrap(), Signs::RightBracket);

        assert_eq!(Signs::from("||").unwrap(), Signs::DoubleVBar);
        assert_eq!(Signs::from("&&").unwrap(), Signs::DoubleAmper);

        assert_eq!(Signs::from("\n").unwrap(), Signs::EndOfLine);
        assert_eq!(Signs::from("<<EOF>>").unwrap(), Signs::EndOfFile);
    }

    #[test]
    fn test_signs_to_string() {
        assert_eq!(Signs::Dot.to_string(), ".");
        assert_eq!(Signs::Comma.to_string(), ",");

        assert_eq!(Signs::Colon.to_string(), ":");
        assert_eq!(Signs::Semicolon.to_string(), ";");

        assert_eq!(Signs::Equal.to_string(), "=");
        assert_eq!(Signs::DoubleEqual.to_string(), "==");
        assert_eq!(Signs::EqualGreater.to_string(), "=>");

        assert_eq!(Signs::Not.to_string(), "!");
        assert_eq!(Signs::NotEqual.to_string(), "!=");

        assert_eq!(Signs::Plus.to_string(), "+");
        assert_eq!(Signs::PlusEqual.to_string(), "+=");

        assert_eq!(Signs::Minus.to_string(), "-");
        assert_eq!(Signs::MinusEqual.to_string(), "-=");

        assert_eq!(Signs::Less.to_string(), "<");
        assert_eq!(Signs::LessEqual.to_string(), "<=");

        assert_eq!(Signs::Greater.to_string(), ">");
        assert_eq!(Signs::GreaterEqual.to_string(), ">=");

        assert_eq!(Signs::LeftParentheses.to_string(), "(");
        assert_eq!(Signs::RightParentheses.to_string(), ")");

        assert_eq!(Signs::LeftBrace.to_string(), "{");
        assert_eq!(Signs::RightBrace.to_string(), "}");

        assert_eq!(Signs::LeftBracket.to_string(), "[");
        assert_eq!(Signs::RightBracket.to_string(), "]");

        assert_eq!(Signs::DoubleVBar.to_string(), "||");
        assert_eq!(Signs::DoubleAmper.to_string(), "&&");

        assert_eq!(Signs::EndOfLine.to_string(), "\n");
        assert_eq!(Signs::EndOfFile.to_string(), "<<EOF>>");
    }
}
