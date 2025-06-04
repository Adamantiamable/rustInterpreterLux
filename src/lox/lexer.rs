use std::io::BufRead;
use std::fs;
use std::io::Write;
use crate::lox::error_manager::ErrorManager;
use crate::lox::scanner;
use crate::lox::parser::Parser;
use crate::lox::interpreter::Interpreter;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Lexer {
    error_reporter: Rc<RefCell<ErrorManager>>,
}

impl Lexer {
    pub fn new() -> Self {
        Self { error_reporter: Rc::new(RefCell::new(ErrorManager::new())) }
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
        self.error_reporter.borrow_mut().had_error = false; // Reset error state
        // Here you would typically parse and interpret the source code.
        // For now, we just print it to demonstrate that it was read.
        println!("Running Lox code:\n{}", source.clone());
        let mut error_reporter = self.error_reporter.clone();
        let mut scanner = scanner::Scanner::new(source.clone(), error_reporter.clone());
        let mut tokens = scanner.scan_tokens();
        let mut parser = crate::lox::parser::Parser::new(tokens, error_reporter.clone());
        println!("Starting parsing");
        let mut statements = parser.parse();
        println!("Finished parsing");
        let mut interpreter = Interpreter::new(error_reporter.clone());
        interpreter.interpret(statements);
        if error_reporter.borrow_mut().had_error {
            eprintln!("Errors encountered during parsing or interpretation.");
            std::process::exit(65); // Exit with error code
        } else {
            println!("Lox code executed successfully.");
        }       
    }

}