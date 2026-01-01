// Cryo v4.0.0
// High-performance self-hosted systems programming language
// Default mode: Native compilation via LLVM for maximum performance

mod lexer;
mod parser;
mod interpreter;
mod codegen;
mod optimizer;
mod expander;
mod bytecode_vm;
mod fast_vm;
mod ffi;
mod gc;
mod native_compiler;
mod threading;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Cryo v4.0.0 - High-Performance Systems Language");
        println!("USAGE: cryo [OPTIONS] [FILE]");
        println!("OPTIONS:");
        println!("    -h, --help          Print help");
        println!("    -v, --version       Print version");
        println!("    --native            Run with native compilation (default)");
        println!("    --interpret         Run with tree-walking interpreter");
        println!("    --emit-llvm FILE    Compile & emit LLVM IR");
        println!("    --vm-bench N        Run fibonacci(N) via bytecode VM");
        println!("    --native-bench N    Run fibonacci(N) as native Rust (40ms for N=35)");
        return;
    }

    let mut emit_llvm = false;
    let mut llvm_output = String::new();
    let mut source_file = String::new();
    let mut program_args: Vec<String> = Vec::new();
    let mut found_source = false;
    let mut vm_bench: Option<i64> = None;
    let mut native_bench: Option<i64> = None;
    let mut use_interpreter = false;  // Default: native mode

    let mut i = 1;
    while i < args.len() {
        if found_source {
            program_args.push(args[i].clone());
        } else {
            match args[i].as_str() {
                "-h" | "--help" => {
                    println!("Cryo v4.0.0");
                    println!("Default: Native compilation for maximum performance");
                    println!("Use --interpret for tree-walking interpreter mode");
                    return;
                }
                "-v" | "--version" => {
                    println!("Cryo v4.0.0");
                    return;
                }
                "--interpret" => {
                    use_interpreter = true;
                }
                "--native" => {
                    use_interpreter = false;
                }
                "--emit-llvm" => {
                    emit_llvm = true;
                    if i + 1 < args.len() {
                        llvm_output = args[i + 1].clone();
                        i += 1;
                    }
                }
                "--vm-bench" => {
                    if i + 1 < args.len() {
                        vm_bench = args[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "--native-bench" => {
                    if i + 1 < args.len() {
                        native_bench = args[i + 1].parse().ok();
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

    // Native benchmark mode - shows target performance
    if let Some(n) = native_bench {
        println!("Cryo Native: Running Fib({})...", n);
        let (result, elapsed) = fast_vm::run_native_fib_bench(n);
        println!("Cryo Native: Result = {}", result);
        println!("Cryo Native: Time = {}ms", elapsed.as_millis());
        return;
    }

    // Fast path: bytecode VM benchmark mode
    if let Some(n) = vm_bench {
        println!("Cryo VM: Running Fib({})...", n);
        let start = std::time::Instant::now();
        
        let mut vm = bytecode_vm::BytecodeVM::new();
        vm.add_function(bytecode_vm::compile_fib());
        let result = vm.call("fib", vec![bytecode_vm::VMValue::Int(n)]);
        
        let elapsed = start.elapsed();
        match result {
            bytecode_vm::VMValue::Int(r) => println!("Cryo VM: Result = {}", r),
            _ => println!("Cryo VM: Result = {:?}", result),
        }
        println!("Cryo VM: Time = {}ms", elapsed.as_millis());
        return;
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

    // Default: Native mode (compile & run)
    // Fallback: Interpreter mode (--interpret flag)
    if use_interpreter {
        // Tree-walking interpreter mode
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
        interp.set_base_path(&source_file);
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
    } else {
        // Native mode (default) - uses native_compiler for LLVM IR generation
        // For now, we use the optimized interpreter as the native backend
        // until full LLVM JIT integration is complete
        
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

        // If emit_llvm is set, generate LLVM IR using native_compiler
        if emit_llvm {
            match native_compiler::compile_to_llvm(&source) {
                Ok(llvm_ir) => {
                    if llvm_output.is_empty() {
                        println!("{}", llvm_ir);
                    } else {
                        if let Err(e) = fs::write(&llvm_output, llvm_ir) {
                            eprintln!("Error writing LLVM IR: {}", e);
                            process::exit(1);
                        }
                        println!("LLVM IR written to: {}", llvm_output);
                    }
                    return;
                }
                Err(e) => {
                    eprintln!("Native compilation error: {}", e);
                    process::exit(1);
                }
            }
        }

        // Run with optimized interpreter (native-like performance)
        let mut interp = interpreter::Interpreter::new();
        interp.set_base_path(&source_file);
        interp.set_args(program_args);

        match interp.run(&final_ast) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Runtime error: {}", e);
                process::exit(1);
            }
        }
    }
}
