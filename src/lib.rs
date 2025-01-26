use std::fs;
use std::io::{self, BufRead, Write};

/// file reading logic
pub fn run_file(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    run(&content)?;
    Ok(())
}

/// intimate convo with the interpreter logic
pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();

    loop {
        print!("> ");
        
        // ensure the prompt is displayed immediately
        io::stdout().flush()?;

        let mut line = String::new();
        
        // reads a line of input from the user from the cmdline 
        // and returns a Result type, hence `?` 
        //
        // locking ensures that no other part of the program can read from 
        // stdin at the same time, preventing race conditions in multi-threaded programs.
        stdin.lock().read_line(&mut line)?;
        
        if line.trim().is_empty() {
            break;
        }
        
        run(&line)?;
    }

    Ok(())
}

pub fn run(source: &str) -> io::Result<()> {
    Ok(())
}