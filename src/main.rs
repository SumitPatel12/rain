use std::env;

use anyhow::{anyhow, Result};
use lox_interpreter::Lox;

pub mod lox_interpreter;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    println!("{:#?}", args);

    if args.len() > 2 {
        return Err(anyhow!("Usage: jlox [script]"));
    } else if args.len() == 2 {
        let _ = lox.run_file(args[1].clone());
    } else {
        let _ = lox.run_prompt();
    }
    Ok(())
}
