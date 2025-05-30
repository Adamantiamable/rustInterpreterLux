mod lox;
mod tool;

fn main() {
    let mut interpreter = lox::lexer::Lexer::new();
   interpreter.main();
   tool::ast_printer::main();
}