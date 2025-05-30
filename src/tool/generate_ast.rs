struct GenerateAst {
    pub ast: String,
}

#[derive(Debug)]
pub enum Expr{
    Binary{ // 1+2
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Grouping{ // (xxx)
        expression: Box<Expr>,
    },
    Literal{ // 1, "hello", true, false, nil
        value: LiteralValue,
    },
    Unary{ // -1, !true
        operator: String,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}



impl GenerateAst {
    pub fn new() -> GenerateAst {
        GenerateAst {
            ast: String::new(),
        }          
    }



    pub fn main(&mut self, args: Vec<String>) {
        if args.len() != 1 {
            eprintln!("Usage: generate_ast <output file>");
            std::process::exit(1);
            }
        else {
            let output_file = &args[0];
            println!("Output file specified: {}", output_file);
        }
    }
}