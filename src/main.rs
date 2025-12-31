// Argon Interpreter v2.24.0
// Rust implementation that can run Argon source files

mod lexer;
mod parser;
mod interpreter;
mod codegen;
mod optimizer;
mod expander;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Argon - A Memory-Safe Systems Language");
        println!("USAGE: argon [OPTIONS] [FILE]");
        println!("OPTIONS:");
        println!("    -h, --help        Print help");
        println!("    -v, --version     Print version");
        println!("    --emit-llvm FILE  Compile & emit LLVM IR");
        return;
    }

    let mut emit_llvm = false;
    let mut llvm_output = String::new();
    let mut source_file = String::new();
    let mut program_args: Vec<String> = Vec::new();
    let mut found_source = false;

    let mut i = 1;
    while i < args.len() {
        if found_source {
            program_args.push(args[i].clone());
        } else {
            match args[i].as_str() {
                "-h" | "--help" => {
                    println!("Argon Interpreter v2.24.0");
                    return;
                }
                "-v" | "--version" => {
                    println!("Argon Interpreter v2.24.0");
                    return;
                }
                "--emit-llvm" => {
                    emit_llvm = true;
                    if i + 1 < args.len() {
                        llvm_output = args[i + 1].clone();
                        i += 1;
                    }
                }
                _ => {
                    source_file = args[i].clone();
                    found_source = true;
                    program_args.insert(0, source_file.clone());
                }
            }
        }
        i += 1;
    }

    if source_file.is_empty() {
        eprintln!("Error: No source file specified");
        process::exit(1);
    }

    let source = match fs::read_to_string(&source_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading '{}': {}", source_file, e);
            process::exit(1);
        }
    };

    let tokens = lexer::tokenize(&source);
    let mut parser = parser::Parser::new(tokens);
    
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };

    // Macro Expansion Pass
    let mut expander = expander::Expander::new();
    let expanded_ast = expander.expand(ast);

    let optimizer = crate::optimizer::Optimizer::new();
    let final_ast = optimizer.optimize(expanded_ast);

    let mut interp = interpreter::Interpreter::new();
    interp.set_base_path(&source_file); // Set base path for relative imports
    if emit_llvm {
        interp.set_emit_llvm(true, &llvm_output);
    }
    interp.set_args(program_args);

    match interp.run(&final_ast) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Runtime error: {}", e);
            process::exit(1);
        }
    }
}
