// Copyright (C) 2021 Wondwise Authors. All rights reserved. GPL-3.0 license.

mod keywords;
mod signs;

pub use keywords::*;
pub use signs::*;

#[allow(dead_code)] // TODO: remove this when no longer needed
pub enum Token {
    Identifier(String),
    String(String),
    Number(f64),

    Keyword(Keywords),
    Sign(Signs),
}
