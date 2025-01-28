use std::{fs, io, io::Write};

use anyhow::{anyhow, Result};
use scanner::Scanner;

pub mod ast_tools;
pub mod scanner;
pub mod token;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, file_name: String) -> Result<()> {
        let file_contents = fs::read(file_name)?;
        self.run(file_contents)?;

        if self.had_error {
            return Err(anyhow!("Error"));
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
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

    pub fn run(&self, source: Vec<u8>) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        println!("{:#?}", tokens);
        if self.had_error {
            return Err(anyhow!("Error"));
        }
        Ok(())
    }

    pub fn error(line: usize, column: usize, message: String) {
        println!("[line: {}] Error {}: {}", line, column, message);
    }
}
