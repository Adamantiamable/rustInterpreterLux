use std::io::BufRead;
use std::fs;
use std::io::Write;
use crate::lox::error_manager::ErrorManager;
use crate::lox::scanner;
use crate::lox::parser::Parser;
use crate::lox::interpreter::Interpreter;

pub struct Lexer {
    error_reporter: ErrorManager,
}

impl Lexer {
    pub fn new() -> Self {
        Self { error_reporter: ErrorManager::new() }
    }

    pub fn main(&mut self) {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            1 => self.run_prompt(),
            2 => self.run_file(&args[1]),
            _ => {
                eprintln!("Usage: lox [script]");
                std::process::exit(64);
            }
        }

    }

    fn run_file(&mut self, path: &str) {
        let bytes = fs::read(path).expect("Failed to read file");
        let source = String::from_utf8(bytes).expect("Invalid UTF-8 in file");
        self.run(source);
    }

    fn run_prompt(&mut self) {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let mut lines = stdin.lock().lines();

        loop {
            print!(">");
            stdout.flush().expect("Failed to flush stdout");

            // Read a line from stdin
            let line = match lines.next() {
                Some(Ok(line)) => line,
                Some(Err(e)) => {
                    eprintln!("Error reading line: {}", e);
                    continue;
                }
                None => break, // EOF reached
            };
            if line.is_empty() {
                continue; // Skip empty lines
            }
            self.run(line);
        }
    }

    fn run(&mut self, source: String) {
        self.error_reporter.had_error = false; // Reset error state
        // Here you would typically parse and interpret the source code.
        // For now, we just print it to demonstrate that it was read.
        println!("Running Lox code:\n{}", source.clone());
        let mut scanner = scanner::Scanner::new(source.clone(), &mut self.error_reporter);
        let mut tokens = scanner.scan_tokens();
        let mut parser = crate::lox::parser::Parser::new(tokens, &mut self.error_reporter);
        let expr = match parser.parse() {
            Ok(expr) => expr,
            Err(_) => {
                eprintln!("Error during parsing. Exiting.");
                return;
            }
        };
        if self.error_reporter.had_error {
            eprintln!("Error during parsing. Exiting.");
            return;
        }
        println!("Parsed expression: {:?}", &expr);
        let mut interpreter = Interpreter::new(&mut self.error_reporter);
        match interpreter.evaluate(&expr) {
            Ok(result) => println!("Result: {:?}", result),
            Err(e) => eprintln!("Runtime error: {:?}", e),
        }
    }

}