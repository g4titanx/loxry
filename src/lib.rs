use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Display;

#[derive(Debug)]
pub enum LoxError {
    ParseError { line: usize, message: String },
    IoError(io::Error),
}

// allows converting io::Error to LoxError
impl From<io::Error> for LoxError {
    fn from(error: io::Error) -> Self {
        LoxError::IoError(error)
    }
}

/// file reading logic
pub fn run_file(path: &str) -> Result<(), LoxError> {
    let content = fs::read_to_string(path)?;
    run(&content)?;
    Ok(())
}

/// interactive prompt logic
/// REPL -> Read a line of input, Evaluate it,
/// Print the result, then loop and do it all over again
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

fn run(source: &str) -> io::Result<()> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    
    for token in tokens {
        println!("{}", token);
    }
    
    Ok(())
}

struct Scanner {
    source: String,
    // some other field
}

impl Scanner {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
        }
    }
    
    fn scan_tokens(&self) -> Result<Vec<Token>, LoxError> {
        // scanning implementatio...
        // returns Result instead of using global error state
        todo!("Implement token scanning")
    }
}

#[derive(Debug)]
pub struct Token {
    // token fields
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // implement token display
        todo!("Implement token display")
    }
}