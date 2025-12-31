// Argon Interpreter v2.19.0
// Rust implementation that can run Argon source files
// Supports: functions, let, if/else, while, arrays, structs, etc.

mod lexer;
mod parser;
mod interpreter;
mod codegen;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Argon - A Memory-Safe Systems Language");
        println!();
        println!("USAGE:");
        println!("    argon [OPTIONS] [FILE]");
        println!();
        println!("OPTIONS:");
        println!("    -h, --help        Print this help message");
        println!("    -v, --version     Print version information");
        println!("    --emit-llvm FILE  Compile to LLVM IR");
        println!();
        println!("    argon                           Run internal test");
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
            // All args after source file are program args
            program_args.push(args[i].clone());
        } else {
            match args[i].as_str() {
                "-h" | "--help" => {
                    println!("Argon Interpreter v2.21.0");
                    println!("Runs Argon source files or compiles to LLVM IR");
                    return;
                }
                "-v" | "--version" => {
                    println!("Argon Interpreter v2.21.0");
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
                    // Add source file as arg[0] for the program
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
    
    // Read source file
    let source = match fs::read_to_string(&source_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error: Could not read file '{}': {}", source_file, e);
            process::exit(1);
        }
    };
    
    // Tokenize
    let tokens = lexer::tokenize(&source);
    
    // Parse
    let ast = match parser::parse(&tokens) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };
    
    if emit_llvm {
        // Generate LLVM IR mode - run the compiler and capture output
        let mut interp = interpreter::Interpreter::new();
        interp.set_emit_llvm(true, &llvm_output);
        interp.set_args(program_args.clone());
        match interp.run(&ast) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Runtime error: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Normal interpretation mode
        let mut interp = interpreter::Interpreter::new();
        interp.set_args(program_args);
        match interp.run(&ast) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Runtime error: {}", e);
                process::exit(1);
            }
        }
    }
}
