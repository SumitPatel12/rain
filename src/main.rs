use std::env;

use anyhow::{anyhow, Result};
use lox_interpreter::Lox;

pub mod lox_interpreter;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();

    if args.len() > 1 {
        return Err(anyhow!("Usage: jlox [script]"));
    } else if args.len() == 1 {
        let _ = lox.run_file(args[0].clone());
    } else {
        let _ = lox.run_prompt();
    }
    Ok(())
}
