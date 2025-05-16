use crate::token::{Keyword, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
    reached_end: bool, // EOF flag
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.next();
        Self {
            input: chars,
            current_char,
            reached_end: false, // EOF flag
        }
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        match number.parse::<u64>() {
            Ok(n) => Token::Number(n),
            Err(_) => Token::Invalid('0'),
        }
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::new();
        
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        // Check if it's a keyword
        match identifier.to_uppercase().as_str() {
            "SELECT" => Token::Keyword(Keyword::Select),
            "CREATE" => Token::Keyword(Keyword::Create),
            "TABLE" => Token::Keyword(Keyword::Table),
            "WHERE" => Token::Keyword(Keyword::Where),
            "ORDER" => Token::Keyword(Keyword::Order),
            "BY" => Token::Keyword(Keyword::By),
            "ASC" => Token::Keyword(Keyword::Asc),
            "DESC" => Token::Keyword(Keyword::Desc),
            "FROM" => Token::Keyword(Keyword::From),
            "AND" => Token::Keyword(Keyword::And),
            "OR" => Token::Keyword(Keyword::Or),
            "NOT" => Token::Keyword(Keyword::Not),
            "TRUE" => Token::Keyword(Keyword::True),
            "FALSE" => Token::Keyword(Keyword::False),
            "PRIMARY" => Token::Keyword(Keyword::Primary),
            "KEY" => Token::Keyword(Keyword::Key),
            "CHECK" => Token::Keyword(Keyword::Check),
            "INT" => Token::Keyword(Keyword::Int),
            "BOOL" => Token::Keyword(Keyword::Bool),
            "VARCHAR" => Token::Keyword(Keyword::Varchar),
            "NULL" => Token::Keyword(Keyword::Null),
            "NOT NULL" => Token::Keyword(Keyword::Null), // This won't work as is, will handle "NOT NULL" differently
            _ => Token::Identifier(identifier),
        }
    }

    fn read_string(&mut self, quote_char: char) -> Result<Token, String> {
        let mut string_value = String::new();
        self.advance(); // Skip the opening quote
        
        while let Some(c) = self.current_char {
            if c == '\'' || c == '"' {
                if c != quote_char {
                    // Advance past the mismatched quote to prevent double error
                    self.advance();
                    return Err(format!("Mismatched quotes: string started with {} but found {}", quote_char, c));
                }
                self.advance();
                return Ok(Token::String(string_value));
            } else {
                string_value.push(c);
                self.advance();
            }
        }
        
        Err(format!("Unterminated string starting with {}", quote_char))
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        
        if let Some(current) = self.current_char {
            let token = match current {
                '0'..='9' => Ok(self.read_number()),
                'a'..='z' | 'A'..='Z' | '_' => Ok(self.read_identifier_or_keyword()),
                '"' | '\'' => self.read_string(current),
                '(' => {
                    self.advance();
                    Ok(Token::LeftParentheses)
                },
                ')' => {
                    self.advance();
                    Ok(Token::RightParentheses)
                },
                ',' => {
                    self.advance();
                    Ok(Token::Comma)
                },
                ';' => {
                    self.advance();
                    Ok(Token::Semicolon)
                },
                '>' => {
                    self.advance();
                    if let Some('=') = self.current_char {
                        self.advance();
                        Ok(Token::GreaterThanOrEqual)
                    } else {
                        Ok(Token::GreaterThan)
                    }
                },
                '<' => {
                    self.advance();
                    if let Some('=') = self.current_char {
                        self.advance();
                        Ok(Token::LessThanOrEqual)
                    } else {
                        Ok(Token::LessThan)
                    }
                },
                '=' => {
                    self.advance();
                    Ok(Token::Equal)
                },
                '!' => {
                    self.advance();
                    if let Some('=') = self.current_char {
                        self.advance();
                        Ok(Token::NotEqual)
                    } else {
                        Ok(Token::Invalid('!'))
                    }
                },
                '*' => {
                    self.advance();
                    Ok(Token::Star)
                },
                '/' => {
                    self.advance();
                    Ok(Token::Divide)
                },
                '+' => {
                    self.advance();
                    Ok(Token::Plus)
                },
                '-' => {
                    self.advance();
                    Ok(Token::Minus)
                },
                _ => {
                    self.advance();
                    Ok(Token::Invalid(current))
                }
            };
            token
        } else {
            Ok(Token::Eof)
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, String>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // If we've already reached the end, stop iteration
        if self.reached_end {
            return None;
        }
        
        match self.next_token() {
            Ok(Token::Eof) => {
                // Mark that we've reached the end
                self.reached_end = true;
                // Return Eof token
                Some(Ok(Token::Eof))
            },
            Ok(token) => Some(Ok(token)),
            Err(e) => Some(Err(e)),
        }
    }
}