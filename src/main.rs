use std::env;
use loxry::LoxError;
use loxry::{run_file, run_prompt};

fn main() -> Result<(), LoxError> {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        // uses args[1] because args[0] is the program name itself
        // for example: ["./loxry", "script.lox"]
        2 => run_file(&args[1])?,
        1 => run_prompt()?,
        _ => {
            eprintln!("Usage: loxry [script]");
            std::process::exit(64);
        }
    }
    
    Ok(())
}