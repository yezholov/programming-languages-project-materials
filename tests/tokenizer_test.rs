use programming_languages_project_kyrylo_yezholov::{
    Token, Keyword,
    Tokenizer
};
#[test]
fn test_basic_select() {
    let input = "SELECT name, age FROM users;";
    let tokens: Vec<Token> = Tokenizer::new(input)
        .collect::<Result<Vec<Token>, String>>()
        .unwrap();
    
    assert_eq!(tokens, vec![
        Token::Keyword(Keyword::Select),
        Token::Identifier("name".to_string()),
        Token::Comma,
        Token::Identifier("age".to_string()),
        Token::Keyword(Keyword::From),
        Token::Identifier("users".to_string()),
        Token::Semicolon,
        Token::Eof
    ]);
}

#[test]
fn test_numbers() {
    let input = "123 456 789";
    let tokens: Vec<Token> = Tokenizer::new(input)
        .collect::<Result<Vec<Token>, String>>()
        .unwrap();
    
    assert_eq!(tokens, vec![
        Token::Number(123),
        Token::Number(456),
        Token::Number(789),
        Token::Eof
    ]);
}

#[test]
fn test_strings() {
    let input = "'hello' \"world\"";
    let tokens: Vec<Token> = Tokenizer::new(input)
        .collect::<Result<Vec<Token>, String>>()
        .unwrap();
    
    assert_eq!(tokens, vec![
        Token::String("hello".to_string()),
        Token::String("world".to_string()),
        Token::Eof
    ]);
}

#[test]
fn test_operators() {
    let input = "< <= > >= = != + - * /";
    let tokens: Vec<Token> = Tokenizer::new(input)
        .collect::<Result<Vec<Token>, String>>()
        .unwrap();
    
    assert_eq!(tokens, vec![
        Token::LessThan,
        Token::LessThanOrEqual,
        Token::GreaterThan,
        Token::GreaterThanOrEqual,
        Token::Equal,
        Token::NotEqual,
        Token::Plus,
        Token::Minus,
        Token::Star,
        Token::Divide,
        Token::Eof
    ]);
}

#[test]
fn test_keywords() {
    let input = "SELECT CREATE TABLE WHERE ORDER BY ASC DESC FROM";
    let tokens: Vec<Token> = Tokenizer::new(input)
        .collect::<Result<Vec<Token>, String>>()
        .unwrap();
    
    assert_eq!(tokens, vec![
        Token::Keyword(Keyword::Select),
        Token::Keyword(Keyword::Create),
        Token::Keyword(Keyword::Table),
        Token::Keyword(Keyword::Where),
        Token::Keyword(Keyword::Order),
        Token::Keyword(Keyword::By),
        Token::Keyword(Keyword::Asc),
        Token::Keyword(Keyword::Desc),
        Token::Keyword(Keyword::From),
        Token::Eof
    ]);
}

#[test]
fn test_unclosed_string() {
    let input = "'unclosed string";
    let result = Tokenizer::new(input).collect::<Result<Vec<Token>, String>>();
    assert!(result.is_err());
}

#[test]
fn test_invalid_number() {
    let input = "12a34";
    let tokens: Vec<Token> = Tokenizer::new(input)
        .collect::<Result<Vec<Token>, String>>()
        .unwrap();
    
    // Tokenizer reads "12" as a number and "a34" as an identifier
    assert_eq!(tokens, vec![
        Token::Number(12),
        Token::Identifier("a34".to_string()),
        Token::Eof
    ]);
}

#[test]
fn test_invalid_operator() {
    let input = "@";
    let tokens: Vec<Token> = Tokenizer::new(input)
        .collect::<Result<Vec<Token>, String>>()
        .unwrap();
    
    // Tokenizer returns Invalid token for unknown character
    assert_eq!(tokens, vec![
        Token::Invalid('@'),
        Token::Eof
    ]);
}

#[test]
fn test_empty_input() -> Result<(), String> {
    let input = "";
    let tokens = Tokenizer::new(input).collect::<Result<Vec<Token>, String>>()?;
    assert_eq!(tokens, vec![Token::Eof]);
    Ok(())
}

#[test]
fn test_string_with_newline() {
    let input = "'string with\nnewline'";
    let tokens: Vec<Token> = Tokenizer::new(input)
        .collect::<Result<Vec<Token>, String>>()
        .unwrap();
    
    assert_eq!(tokens, vec![
        Token::String("string with\nnewline".to_string()),
        Token::Eof
    ]);
} 