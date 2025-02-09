use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Token {
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(u64),
    Invalid(char),
    RightParentheses,
    LeftParentheses,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Star,
    Divide,
    Minus,
    Plus,
    Comma,
    Semicolon,
    Eof,
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Keyword {
    Select,
    Create,
    Table,
    Where,
    Order,
    By,
    Asc,
    Desc,
    From,
    And,
    Or,
    Not,
    True,
    False,
    Primary,
    Key,
    Check,
    Int,
    Bool,
    Varchar,
    Null,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(f, "{:?}", keyword),
            Token::Identifier(iden) => write!(f, "{:?}", iden),
            Token::String(str) => write!(f, "{:?}", str),
            Token::Number(num) => write!(f, "{:?}", num),
            Token::RightParentheses => write!(f, "("),
            Token::LeftParentheses => write!(f, ")"),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanOrEqual => write!(f, ">="),
            Token::LessThan => write!(f, "<"),
            Token::LessThanOrEqual => write!(f, "<="),
            Token::Equal => write!(f, "="),
            Token::NotEqual => write!(f, "!="),
            Token::Star => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Semicolon => write!(f, ";"),
            Token::Eof => write!(f, "Eof"),
            Token::Invalid(c) => write!(f, "{}", c),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&Keyword as Into<&str>>::into(self).to_uppercase())
    }
}