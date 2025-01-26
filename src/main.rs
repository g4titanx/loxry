use std::env;
use std::io;
use loxry::{run_file, run_prompt};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        // uses args[1] because args[0] is the program name itself
        // for example: ./loxry script.lox
        // 
        // a reason for calling a `&` to args[1] is that we 
        // transform String to &str 
        2 => run_file(&args[1])?,
        1 => run_prompt()?,
        _ => {
            eprintln!("Usage: loxry [script]");
            std::process::exit(64);
        }
    }
    
    Ok(())
}