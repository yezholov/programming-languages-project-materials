use programming_languages_project_kyrylo_yezholov::{
    Tokenizer,
    Parser,
    Statement, Expression, TableColumn, DBType,
    Constraint, BinaryOperator, UnaryOperator
};
fn parse_expression(input: &str) -> Result<Expression, String> {
    let tokenizer = Tokenizer::new(input);
    Parser::new(tokenizer).and_then(|mut parser| parser.parse_expression(0))
}

fn parse_sql(input: &str) -> Result<Statement, String> {
    let tokenizer = Tokenizer::new(input);
    Parser::new(tokenizer).and_then(|mut parser| parser.parse_statement())
}

#[test]
fn test_simple_binary_operation() {
    let expr = parse_expression("5 + 3").unwrap();
    assert_eq!(expr, Expression::BinaryOperation {
        left_operand: Box::new(Expression::Number(5)),
        operator: BinaryOperator::Plus,
        right_operand: Box::new(Expression::Number(3))
    });
}

#[test]
fn test_operator_precedence() {
    let expr = parse_expression("2 + 3 * 4").unwrap();
    assert_eq!(expr, Expression::BinaryOperation {
        left_operand: Box::new(Expression::Number(2)),
        operator: BinaryOperator::Plus,
        right_operand: Box::new(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Number(3)),
            operator: BinaryOperator::Multiply,
            right_operand: Box::new(Expression::Number(4))
        })
    });
}

#[test]
fn test_parentheses() {
    let expr = parse_expression("(2 + 3) * 4").unwrap();
    assert_eq!(expr, Expression::BinaryOperation {
        left_operand: Box::new(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Number(2)),
            operator: BinaryOperator::Plus,
            right_operand: Box::new(Expression::Number(3))
        }),
        operator: BinaryOperator::Multiply,
        right_operand: Box::new(Expression::Number(4))
    });
}

#[test]
fn test_unary_operation() {
    let expr = parse_expression("-5").unwrap();
    assert_eq!(expr, Expression::UnaryOperation {
        operand: Box::new(Expression::Number(5)),
        operator: UnaryOperator::Minus
    });
}

#[test]
fn test_complex_expression() {
    let expr = parse_expression("(5 - x) < (4 + y)").unwrap();
    assert_eq!(expr, Expression::BinaryOperation {
        left_operand: Box::new(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Number(5)),
            operator: BinaryOperator::Minus,
            right_operand: Box::new(Expression::Identifier("x".to_string()))
        }),
        operator: BinaryOperator::LessThan,
        right_operand: Box::new(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Number(4)),
            operator: BinaryOperator::Plus,
            right_operand: Box::new(Expression::Identifier("y".to_string()))
        })
    });
}

#[test]
fn test_boolean_expression() {
    let expr = parse_expression("x > 5 AND y < 10").unwrap();
    assert_eq!(expr, Expression::BinaryOperation {
        left_operand: Box::new(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Identifier("x".to_string())),
            operator: BinaryOperator::GreaterThan,
            right_operand: Box::new(Expression::Number(5))
        }),
        operator: BinaryOperator::And,
        right_operand: Box::new(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Identifier("y".to_string())),
            operator: BinaryOperator::LessThan,
            right_operand: Box::new(Expression::Number(10))
        })
    });
}

#[test]
fn test_invalid_expression() {
    let result = parse_expression("5 + ");
    assert!(result.is_err());
}

#[test]
fn test_simple_select() {
    let stmt = parse_sql("SELECT name, age FROM users;").unwrap();
    assert_eq!(stmt, Statement::Select {
        columns: vec![
            Expression::Identifier("name".to_string()),
            Expression::Identifier("age".to_string())
        ],
        from: "users".to_string(),
        r#where: None,
        orderby: vec![]
    });
}

#[test]
fn test_select_with_where() {
    let stmt = parse_sql("SELECT id FROM users WHERE age > 18;").unwrap();
    assert_eq!(stmt, Statement::Select {
        columns: vec![Expression::Identifier("id".to_string())],
        from: "users".to_string(),
        r#where: Some(Expression::BinaryOperation {
            left_operand: Box::new(Expression::Identifier("age".to_string())),
            operator: BinaryOperator::GreaterThan,
            right_operand: Box::new(Expression::Number(18))
        }),
        orderby: vec![]
    });
}

#[test]
fn test_select_with_order_by() {
    let stmt = parse_sql("SELECT id FROM users ORDER BY age DESC;").unwrap();
    assert_eq!(stmt, Statement::Select {
        columns: vec![Expression::Identifier("id".to_string())],
        from: "users".to_string(),
        r#where: None,
        orderby: vec![
            Expression::UnaryOperation {
                operand: Box::new(Expression::Identifier("age".to_string())),
                operator: UnaryOperator::Desc
            }
        ]
    });
}

#[test]
fn test_create_table_simple() {
    let stmt = parse_sql("CREATE TABLE users(id INT, name VARCHAR(255));").unwrap();
    assert_eq!(stmt, Statement::CreateTable {
        table_name: "users".to_string(),
        column_list: vec![
            TableColumn {
                column_name: "id".to_string(),
                column_type: DBType::Int,
                constraints: vec![]
            },
            TableColumn {
                column_name: "name".to_string(),
                column_type: DBType::Varchar(255),
                constraints: vec![]
            }
        ]
    });
}

#[test]
fn test_create_table_with_constraints() {
    let stmt = parse_sql("CREATE TABLE employees(id INT PRIMARY KEY, age INT CHECK(age >= 18));").unwrap();
    assert_eq!(stmt, Statement::CreateTable {
        table_name: "employees".to_string(),
        column_list: vec![
            TableColumn {
                column_name: "id".to_string(),
                column_type: DBType::Int,
                constraints: vec![Constraint::PrimaryKey]
            },
            TableColumn {
                column_name: "age".to_string(),
                column_type: DBType::Int,
                constraints: vec![
                    Constraint::Check(Expression::BinaryOperation {
                        left_operand: Box::new(Expression::Identifier("age".to_string())),
                        operator: BinaryOperator::GreaterThanOrEqual,
                        right_operand: Box::new(Expression::Number(18))
                    })
                ]
            }
        ]
    });
}

#[test]
fn test_invalid_select() {
    // Missing FROM clause
    let result = parse_sql("SELECT id;");
    assert!(result.is_err());
}

#[test]
fn test_invalid_create_table() {
    // Missing semicolon
    let result = parse_sql("CREATE TABLE users(id INT)");
    assert!(result.is_err());
}

#[test]
fn test_unmatched_parentheses() -> Result<(), String> {
    let result = parse_expression("(5 + 3");
    match result {
        Err(e) => {
            assert!(e.contains("Expected closing parenthesis"));
            Ok(())
        },
        Ok(_) => Err("Expected error for unmatched parentheses".to_string())
    }
}

#[test]
fn test_invalid_create_table_column() -> Result<(), String> {
    let result = parse_sql("CREATE TABLE users(id INT, age INVALID);");
    match result {
        Err(e) => {
            assert!(e.contains("Expected data type"));
            Ok(())
        },
        Ok(_) => Err("Expected error for invalid data type".to_string())
    }
}

#[test]
fn test_select_with_complex_where() -> Result<(), String> {
    let stmt = parse_sql("SELECT id FROM users WHERE age >= 18 AND (salary > 50000 OR experience >= 5);")?;
    match stmt {
        Statement::Select { r#where: Some(where_clause), .. } => {
            match where_clause {
                Expression::BinaryOperation { operator: BinaryOperator::And, .. } => Ok(()),
                _ => Err("Expected AND operation in WHERE clause".to_string())
            }
        },
        _ => Err("Expected Select statement".to_string())
    }
}

#[test]
fn test_invalid_order_by() -> Result<(), String> {
    let result = parse_sql("SELECT id FROM users ORDER BY;");
    match result {
        Err(e) => {
            assert!(e.contains("Unexpected token in prefix position"));
            Ok(())
        },
        Ok(_) => Err("Expected error for invalid ORDER BY clause".to_string())
    }
}

#[test]
fn test_select_star() -> Result<(), String> {
    let stmt = parse_sql("SELECT * FROM users;")?;
    
    match stmt {
        Statement::Select { columns, from, r#where, orderby } => {
            assert_eq!(columns, vec![Expression::Wildcard]);
            assert_eq!(from, "users");
            assert!(r#where.is_none());
            assert!(orderby.is_empty());
            Ok(())
        },
        _ => Err("Expected SELECT statement".to_string()),
    }
}

#[test]
fn test_select_star_with_where() -> Result<(), String> {
    let stmt = parse_sql("SELECT * FROM users WHERE age > 18;")?;
    
    match stmt {
        Statement::Select { columns, from, r#where, orderby } => {
            assert_eq!(columns, vec![Expression::Wildcard]);
            assert_eq!(from, "users");
            assert!(r#where.is_some());
            assert!(orderby.is_empty());
            Ok(())
        },
        _ => Err("Expected SELECT statement".to_string()),
    }
}

#[test]
fn test_star_as_multiply_operator() -> Result<(), String> {
    let stmt = parse_sql("SELECT age * 2 FROM users;")?;
    
    match stmt {
        Statement::Select { columns, .. } => {
            assert_eq!(columns, vec![
                Expression::BinaryOperation {
                    left_operand: Box::new(Expression::Identifier("age".to_string())),
                    operator: BinaryOperator::Multiply,
                    right_operand: Box::new(Expression::Number(2))
                }
            ]);
            Ok(())
        },
        _ => Err("Expected SELECT statement".to_string()),
    }
}