// TODO: RIGHT NOW I'M NOT LOOKING AT PERFORMANCE, BUT AT SOME PONITN I SHOULD. DON'T FORGET TO DO
// THAT.
use std::env;

use lox_interpreter::{error::LoxError, Lox};

pub mod lox_interpreter;

fn main() -> Result<(), LoxError> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    println!("{:#?}", args);

    if args.len() > 2 {
        return Err(LoxError::Error("Usage: jlox [script]".to_string()));
    } else if args.len() == 2 {
        let _ = lox.run_file(args[1].clone());
    } else {
        let _ = lox.run_prompt();
    }
    Ok(())
}
