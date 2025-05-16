use crate::statement::{BinaryOperator, Constraint, DBType, Expression, Statement, TableColumn, UnaryOperator};
use crate::token::{Keyword, Token};
use crate::tokenizer::Tokenizer;
use std::iter::Peekable;

pub struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Result<Self, String> {
        let mut tokenizer = tokenizer.peekable();
        let current_token = match tokenizer.next() {
            Some(Ok(token)) => Some(token),
            Some(Err(e)) => return Err(e),
            None => None,
        };
        
        Ok(Self {
            tokenizer,
            current_token,
        })
    }
    
    fn advance_token(&mut self) -> Result<(), String> {
        self.current_token = match self.tokenizer.next() {
            Some(Ok(token)) => Some(token),
            Some(Err(e)) => return Err(e),
            None => None,
        };
        Ok(())
    }
    
    // Gets the precedence of the current token if it's a binary operator
    fn get_precedence(&self) -> u8 {
        if let Some(token) = &self.current_token {
            match token {
                // Postfix ASC/DESC have the lowest active precedence
                Token::Keyword(Keyword::Asc) | Token::Keyword(Keyword::Desc) => 1,
                // Logical operators
                Token::Keyword(Keyword::Or) => 2,
                Token::Keyword(Keyword::And) => 3,
                // Comparisons
                Token::Equal | Token::NotEqual |
                Token::GreaterThan | Token::GreaterThanOrEqual |
                Token::LessThan | Token::LessThanOrEqual => 4,
                // Arithmetic
                Token::Plus | Token::Minus => 5,
                Token::Star | Token::Divide => 6,
                _ => 0, // Default: not an infix operator or end of expression group
            }
        } else {
            0
        }
    }
    
    // Parses a prefix expression (unary operations or primary expressions)
    fn parse_prefix(&mut self) -> Result<Expression, String> {
        if let Some(token) = &self.current_token {
            match token {
                Token::Number(n) => {
                    let value = *n;
                    self.advance_token()?;
                    Ok(Expression::Number(value))
                },
                Token::String(s) => {
                    let value = s.clone();
                    self.advance_token()?;
                    Ok(Expression::String(value))
                },
                Token::Identifier(ident) => {
                    let value = ident.clone();
                    self.advance_token()?;
                    Ok(Expression::Identifier(value))
                },
                Token::Keyword(Keyword::True) => {
                    self.advance_token()?;
                    Ok(Expression::Bool(true))
                },
                Token::Keyword(Keyword::False) => {
                    self.advance_token()?;
                    Ok(Expression::Bool(false))
                },
                Token::Keyword(Keyword::Not) => {
                    self.advance_token()?;
                    let operand = self.parse_expression(6)?; // NOT has high precedence
                    Ok(Expression::UnaryOperation {
                        operand: Box::new(operand),
                        operator: UnaryOperator::Not,
                    })
                },
                Token::Plus => {
                    self.advance_token()?;
                    let operand = self.parse_expression(6)?;
                    Ok(Expression::UnaryOperation {
                        operand: Box::new(operand),
                        operator: UnaryOperator::Plus,
                    })
                },
                Token::Minus => {
                    self.advance_token()?;
                    let operand = self.parse_expression(6)?;
                    Ok(Expression::UnaryOperation {
                        operand: Box::new(operand),
                        operator: UnaryOperator::Minus,
                    })
                },
                Token::LeftParentheses => {
                    self.advance_token()?;
                    let expr = self.parse_expression(0)?;
                    if let Some(Token::RightParentheses) = &self.current_token {
                        self.advance_token()?;
                        Ok(expr)
                    } else {
                        Err("Expected closing parenthesis".to_string())
                    }
                },
                _ => Err(format!("Unexpected token in prefix position: {:?}", token)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }
    
    // Parses an infix expression (binary operations)
    fn parse_infix(&mut self, left: Expression) -> Result<Expression, String> {
        if let Some(token) = &self.current_token {
            match token {
                Token::Plus => {
                    self.advance_token()?;
                    let right = self.parse_expression(5)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Plus,
                        right_operand: Box::new(right),
                    })
                },
                Token::Minus => {
                    self.advance_token()?;
                    let right = self.parse_expression(5)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Minus,
                        right_operand: Box::new(right),
                    })
                },
                Token::Star => {
                    self.advance_token()?;
                    let right = self.parse_expression(6)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Multiply,
                        right_operand: Box::new(right),
                    })
                },
                Token::Divide => {
                    self.advance_token()?;
                    let right = self.parse_expression(6)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Divide,
                        right_operand: Box::new(right),
                    })
                },
                Token::Equal => {
                    self.advance_token()?;
                    let right = self.parse_expression(4)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Equal,
                        right_operand: Box::new(right),
                    })
                },
                Token::NotEqual => {
                    self.advance_token()?;
                    let right = self.parse_expression(4)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::NotEqual,
                        right_operand: Box::new(right),
                    })
                },
                Token::GreaterThan => {
                    self.advance_token()?;
                    let right = self.parse_expression(4)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::GreaterThan,
                        right_operand: Box::new(right),
                    })
                },
                Token::GreaterThanOrEqual => {
                    self.advance_token()?;
                    let right = self.parse_expression(4)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::GreaterThanOrEqual,
                        right_operand: Box::new(right),
                    })
                },
                Token::LessThan => {
                    self.advance_token()?;
                    let right = self.parse_expression(4)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::LessThan,
                        right_operand: Box::new(right),
                    })
                },
                Token::LessThanOrEqual => {
                    self.advance_token()?;
                    let right = self.parse_expression(4)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::LessThanOrEqual,
                        right_operand: Box::new(right),
                    })
                },
                Token::Keyword(Keyword::And) => {
                    self.advance_token()?;
                    let right = self.parse_expression(3)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::And,
                        right_operand: Box::new(right),
                    })
                },
                Token::Keyword(Keyword::Or) => {
                    self.advance_token()?;
                    let right = self.parse_expression(2)?;
                    Ok(Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Or,
                        right_operand: Box::new(right),
                    })
                },
                Token::Keyword(Keyword::Asc) => {
                    self.advance_token()?;
                    Ok(Expression::UnaryOperation {
                        operand: Box::new(left),
                        operator: UnaryOperator::Asc,
                    })
                },
                Token::Keyword(Keyword::Desc) => {
                    self.advance_token()?;
                    Ok(Expression::UnaryOperation {
                        operand: Box::new(left),
                        operator: UnaryOperator::Desc,
                    })
                },
                _ => Err(format!("Unexpected token in infix position: {:?}", token)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }
    
    // The main entry point for the Pratt parser
    pub fn parse_expression(&mut self, precedence: u8) -> Result<Expression, String> {
        // First, parse a prefix expression
        let mut left = self.parse_prefix()?;
        
        // Then, as long as the next operator has a higher precedence than the current one,
        // parse the infix expression and update the left-hand side
        while precedence < self.get_precedence() {
            left = self.parse_infix(left)?;
        }
        
        Ok(left)
    }
    
    // Parse the entire SQL query and return a Statement
    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        if let Some(token) = &self.current_token {
            match token {
                Token::Keyword(Keyword::Select) => self.parse_select_statement(),
                Token::Keyword(Keyword::Create) => self.parse_create_table_statement(),
                _ => Err(format!("Expected SELECT or CREATE, got {:?}", token)),
            }
        } else {
            Err("Empty input".to_string())
        }
    }
    
    // Parse a SELECT statement
    fn parse_select_statement(&mut self) -> Result<Statement, String> {
        // Consume the SELECT keyword
        self.advance_token()?;
        
        // Parse columns (selection expressions)
        let mut columns = Vec::new();
        
        // Special handling for SELECT *
        if let Some(Token::Star) = &self.current_token {
            self.advance_token()?;
            columns.push(Expression::Wildcard);
        } else {
            // Parse first column
            columns.push(self.parse_expression(0)?);
            
            // Parse additional columns separated by commas
            while let Some(Token::Comma) = &self.current_token {
                self.advance_token()?; // Consume comma
                columns.push(self.parse_expression(0)?);
            }
        }
        
        // Check for FROM clause
        if let Some(Token::Keyword(Keyword::From)) = &self.current_token {
            self.advance_token()?; // Consume FROM
        } else {
            return Err("Expected FROM clause in SELECT statement".to_string());
        }
        
        // Parse table name
        let from = if let Some(Token::Identifier(table_name)) = &self.current_token {
            let table = table_name.clone();
            self.advance_token()?;
            table
        } else {
            return Err("Expected table name after FROM".to_string());
        };
        
        // Parse optional WHERE clause
        let r#where = if let Some(Token::Keyword(Keyword::Where)) = &self.current_token {
            self.advance_token()?; // Consume WHERE
            Some(self.parse_expression(0)?)
        } else {
            None
        };
        
        // Parse optional ORDER BY clause
        let mut orderby = Vec::new();
        if let Some(Token::Keyword(Keyword::Order)) = &self.current_token {
            self.advance_token()?; // Consume ORDER
            
            // Check for BY
            if let Some(Token::Keyword(Keyword::By)) = &self.current_token {
                self.advance_token()?; // Consume BY
                
                // Parse first ORDER BY expression
                let expr = self.parse_expression(0)?;
                orderby.push(expr);
                
                // Parse additional ORDER BY expressions separated by commas
                while let Some(Token::Comma) = &self.current_token {
                    self.advance_token()?; // Consume comma
                    let expr = self.parse_expression(0)?;
                    orderby.push(expr);
                }
            } else {
                return Err("Expected BY after ORDER".to_string());
            }
        }
        
        // Check for semicolon
        if let Some(Token::Semicolon) = &self.current_token {
            self.advance_token()?;
        } else {
            return Err("Expected semicolon at the end of the SELECT statement".to_string());
        }
        
        Ok(Statement::Select {
            columns,
            from,
            r#where,
            orderby,
        })
    }
    
    // Parse a CREATE TABLE statement
    fn parse_create_table_statement(&mut self) -> Result<Statement, String> {
        // Consume the CREATE keyword
        self.advance_token()?;
        
        // Check for TABLE keyword
        if let Some(Token::Keyword(Keyword::Table)) = &self.current_token {
            self.advance_token()?;
        } else {
            return Err("Expected TABLE after CREATE".to_string());
        }
        
        // Parse table name
        let table_name = if let Some(Token::Identifier(name)) = &self.current_token {
            let table = name.clone();
            self.advance_token()?;
            table
        } else {
            return Err("Expected table name after CREATE TABLE".to_string());
        };
        
        // Check for opening parenthesis
        if let Some(Token::LeftParentheses) = &self.current_token {
            self.advance_token()?;
        } else {
            return Err("Expected ( after table name".to_string());
        }
        
        // Parse column definitions
        let mut column_list = Vec::new();
        
        // Parse first column
        column_list.push(self.parse_column_definition()?);
        
        // Parse additional columns separated by commas
        while let Some(Token::Comma) = &self.current_token {
            self.advance_token()?; // Consume comma
            column_list.push(self.parse_column_definition()?);
        }
        
        // Check for closing parenthesis
        if let Some(Token::RightParentheses) = &self.current_token {
            self.advance_token()?;
        } else {
            return Err("Expected ) after column definitions".to_string());
        }
        
        // Check for semicolon
        if let Some(Token::Semicolon) = &self.current_token {
            self.advance_token()?;
        } else {
            return Err("Expected semicolon at the end of the CREATE TABLE statement".to_string());
        }
        
        Ok(Statement::CreateTable {
            table_name,
            column_list,
        })
    }
    
    // Parse a column definition
    fn parse_column_definition(&mut self) -> Result<TableColumn, String> {
        // Parse column name
        let column_name = if let Some(Token::Identifier(name)) = &self.current_token {
            let column = name.clone();
            self.advance_token()?;
            column
        } else {
            return Err("Expected column name".to_string());
        };
        
        // Parse column type
        let column_type = self.parse_db_type()?;
        
        // Parse optional constraints
        let mut constraints = Vec::new();
        loop {
            if let Some(token) = &self.current_token {
                match token {
                    Token::Keyword(Keyword::Primary) => {
                        self.advance_token()?;
                        // Check for KEY
                        if let Some(Token::Keyword(Keyword::Key)) = &self.current_token {
                            self.advance_token()?;
                            constraints.push(Constraint::PrimaryKey);
                        } else {
                            return Err("Expected KEY after PRIMARY".to_string());
                        }
                    },
                    Token::Keyword(Keyword::Not) => {
                        self.advance_token()?;
                        // Check for NULL
                        if let Some(Token::Keyword(Keyword::Null)) = &self.current_token {
                            self.advance_token()?;
                            constraints.push(Constraint::NotNull);
                        } else {
                            return Err("Expected NULL after NOT".to_string());
                        }
                    },
                    Token::Keyword(Keyword::Check) => {
                        self.advance_token()?;
                        // Check for opening parenthesis
                        if let Some(Token::LeftParentheses) = &self.current_token {
                            self.advance_token()?;
                            // Parse the check expression
                            let expr = self.parse_expression(0)?;
                            // Check for closing parenthesis
                            if let Some(Token::RightParentheses) = &self.current_token {
                                self.advance_token()?;
                                constraints.push(Constraint::Check(expr));
                            } else {
                                return Err("Expected ) after CHECK expression".to_string());
                            }
                        } else {
                            return Err("Expected ( after CHECK".to_string());
                        }
                    },
                    Token::Comma | Token::RightParentheses => {
                        // End of column definition
                        break;
                    },
                    _ => return Err(format!("Unexpected token in column definition: {:?}", token)),
                }
            } else {
                return Err("Unexpected end of input in column definition".to_string());
            }
        }
        
        Ok(TableColumn {
            column_name,
            column_type,
            constraints,
        })
    }
    
    // Parse a database type
    fn parse_db_type(&mut self) -> Result<DBType, String> {
        if let Some(token) = &self.current_token {
            match token {
                Token::Keyword(Keyword::Int) => {
                    self.advance_token()?;
                    Ok(DBType::Int)
                },
                Token::Keyword(Keyword::Bool) => {
                    self.advance_token()?;
                    Ok(DBType::Bool)
                },
                Token::Keyword(Keyword::Varchar) => {
                    self.advance_token()?;
                    // Check for opening parenthesis
                    if let Some(Token::LeftParentheses) = &self.current_token {
                        self.advance_token()?;
                        // Parse the length
                        if let Some(Token::Number(length)) = &self.current_token {
                            let length = *length as usize;
                            self.advance_token()?;
                            // Check for closing parenthesis
                            if let Some(Token::RightParentheses) = &self.current_token {
                                self.advance_token()?;
                                Ok(DBType::Varchar(length))
                            } else {
                                Err("Expected ) after VARCHAR length".to_string())
                            }
                        } else {
                            Err("Expected number for VARCHAR length".to_string())
                        }
                    } else {
                        Err("Expected ( after VARCHAR".to_string())
                    }
                },
                _ => Err(format!("Expected data type, got {:?}", token)),
            }
        } else {
            Err("Unexpected end of input in type definition".to_string())
        }
    }
}

// Helper function to parse a string into a Statement
pub fn build_statement(input: &str) -> Result<Statement, String> {
    let tokenizer = crate::tokenizer::Tokenizer::new(input);
    let mut parser = Parser::new(tokenizer)?;
    parser.parse_statement()
}