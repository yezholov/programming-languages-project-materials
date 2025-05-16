mod statement;
mod token;
mod tokenizer;
mod parser;

use std::io::{self, Write};
use crate::parser::build_statement;

fn main() {
    println!("SQL Parser CLI");
    println!("Type SQL queries to parse or 'exit' to quit.");
    println!("-------------------------------------------");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Exiting...");
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        println!("\nParsed Statement:");
        match build_statement(input) {
            Ok(statement) => println!("{:#?}", statement),
            Err(e) => println!("Error: {}", e),
        }
    }
}