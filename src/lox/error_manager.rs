#[derive(Debug)]
pub enum Error {
    Runtime(String),
    Other(String),
}

#[derive(Debug)]
pub struct ErrorManager {
    pub had_error: bool,
    pub had_runtime_error: bool,
}


impl ErrorManager {
    pub fn new() -> Self {
        Self { had_error: false, 
        had_runtime_error: false }
    }

    pub fn report(&mut self, line: usize, message: &str, about: Option<&str>) -> Error {
        eprintln!("[line {}] Error: {}", line, message);
        if about.is_some() {
            eprintln!("About: {}", about.unwrap());
        }
        self.had_error = true;
        Error::Other(message.to_string())
    }


    pub fn report_runtime_error(&mut self, message: &str) -> Error {
        eprintln!("Runtime Error: {}", message);
        self.had_runtime_error = true;
        Error::Runtime(message.to_string())
    }
}