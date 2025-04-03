// TODO: ADD FUNCTIONALITY OF BREAK FOR LOOPS.
use std::{fs, io, io::Write};

use error::LoxError;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub mod ast_tools;
pub mod environment;
pub mod error;
pub mod function;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod token;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, file_name: String) -> Result<(), LoxError> {
        let file_contents = fs::read(file_name)?;
        self.run(file_contents)?;

        if self.had_error {
            return Err(LoxError::Error("Had Error".to_string()));
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), LoxError> {
        let mut line = String::new();

        loop {
            print!("> ");
            io::stdout().flush()?;

            let bytes_read = io::stdin().read_line(&mut line)?;

            if bytes_read == 0 {
                break;
            }

            let line = line.trim_end();

            let _ = self.run(line.into());
            // NOTE: If the user gives one wrong prompt we do not want to end their whole session.
            self.had_error = false;
        }
        Ok(())
    }

    pub fn run(&self, source: Vec<u8>) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        //println!("Tokens: {:#?}", tokens);

        let mut parser = Parser::new(tokens);
        // TODO: I should not be escalating the error here, but for now it's fine.
        let statements = parser.parse()?;

        if self.had_error {
            //println!("Got An Error.");
            return Err(LoxError::Error("Error running.".to_string()));
        }

        //println!("Statements: {:#?}", statements);
        let mut intpereter = Interpreter::new();
        intpereter.interpret(statements)?;

        Ok(())
    }

    pub fn error(line: usize, column: usize, message: String) {
        println!("[line: {}] Error {}: {}", line, column, message);
    }
}
