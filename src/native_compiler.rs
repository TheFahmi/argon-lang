// Cryo Native Compiler (Rust)
// Compiles Cryo source directly to LLVM IR
// Much faster than self-hosted compiler.ar

use crate::parser::{Parser, TopLevel, Stmt, Expr, Function};
use crate::lexer;

pub struct Compiler {
    output: String,
    func_counter: usize,
    label_counter: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            output: String::new(),
            func_counter: 0,
            label_counter: 0,
        }
    }

    fn new_label(&mut self) -> String {
        self.label_counter += 1;
        format!("L{}", self.label_counter)
    }

    pub fn compile(&mut self, source: &str) -> Result<String, String> {
        let tokens = lexer::tokenize(source);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("Parse error: {}", e))?;

        // LLVM IR Header
        self.output.push_str("; Cryo Native Compiler Output\n");
        self.output.push_str("target triple = \"x86_64-pc-linux-gnu\"\n\n");
        
        // External declarations
        self.output.push_str("declare i32 @printf(i8*, ...)\n");
        self.output.push_str("declare i64 @time(i64*)\n");
        self.output.push_str("@.str_int = private unnamed_addr constant [5 x i8] c\"%ld\\0A\\00\"\n");
        self.output.push_str("@.str_s = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\"\n\n");

        // Compile all top-level items
        for item in &ast {
            match item {
                TopLevel::Function(f) => self.compile_function(f)?,
                _ => {}
            }
        }

        Ok(self.output.clone())
    }

    fn compile_function(&mut self, func: &Function) -> Result<(), String> {
        let name = &func.name;
        let params: Vec<String> = func.params.iter()
            .map(|p| format!("i64 %{}", p.name))
            .collect();
        
        self.output.push_str(&format!("define i64 @{}({}) {{\n", name, params.join(", ")));
        self.output.push_str("entry:\n");

        // Allocate space for parameters
        for param in &func.params {
            self.output.push_str(&format!("  %{}.addr = alloca i64\n", param.name));
            self.output.push_str(&format!("  store i64 %{}, i64* %{}.addr\n", param.name, param.name));
        }

        // Compile function body
        if let Some(body) = &func.body {
            for stmt in body {
                self.compile_stmt(stmt)?;
            }
        }

        // Default return
        self.output.push_str("  ret i64 0\n");
        self.output.push_str("}\n\n");
        
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let(name, _typ, expr) => {
                self.output.push_str(&format!("  %{}.addr = alloca i64\n", name));
                let val = self.compile_expr(expr)?;
                self.output.push_str(&format!("  store i64 {}, i64* %{}.addr\n", val, name));
            }
            Stmt::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    let val = self.compile_expr(expr)?;
                    self.output.push_str(&format!("  ret i64 {}\n", val));
                } else {
                    self.output.push_str("  ret i64 0\n");
                }
            }
            Stmt::If(cond, then_block, else_block) => {
                let cond_val = self.compile_expr(cond)?;
                let then_label = self.new_label();
                let else_label = self.new_label();
                let end_label = self.new_label();

                self.output.push_str(&format!("  %cmp{} = icmp ne i64 {}, 0\n", self.label_counter, cond_val));
                self.output.push_str(&format!("  br i1 %cmp{}, label %{}, label %{}\n", 
                    self.label_counter, then_label, else_label));

                self.output.push_str(&format!("{}:\n", then_label));
                for s in then_block {
                    self.compile_stmt(s)?;
                }
                self.output.push_str(&format!("  br label %{}\n", end_label));

                self.output.push_str(&format!("{}:\n", else_label));
                if let Some(else_stmts) = else_block {
                    for s in else_stmts {
                        self.compile_stmt(s)?;
                    }
                }
                self.output.push_str(&format!("  br label %{}\n", end_label));

                self.output.push_str(&format!("{}:\n", end_label));
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<String, String> {
        self.func_counter += 1;
        let tmp = format!("%t{}", self.func_counter);

        match expr {
            Expr::Number(n) => Ok(format!("{}", n)),
            Expr::Identifier(name) => {
                self.output.push_str(&format!("  {} = load i64, i64* %{}.addr\n", tmp, name));
                Ok(tmp)
            }
            Expr::BinOp(left, op, right) => {
                let l = self.compile_expr(left)?;
                let r = self.compile_expr(right)?;
                let op_str = match op.as_str() {
                    "+" => "add",
                    "-" => "sub",
                    "*" => "mul",
                    "/" => "sdiv",
                    "%" => "srem",
                    "<" => {
                        self.output.push_str(&format!("  %cmp{} = icmp slt i64 {}, {}\n", self.func_counter, l, r));
                        self.output.push_str(&format!("  {} = zext i1 %cmp{} to i64\n", tmp, self.func_counter));
                        return Ok(tmp);
                    }
                    ">" => {
                        self.output.push_str(&format!("  %cmp{} = icmp sgt i64 {}, {}\n", self.func_counter, l, r));
                        self.output.push_str(&format!("  {} = zext i1 %cmp{} to i64\n", tmp, self.func_counter));
                        return Ok(tmp);
                    }
                    "==" => {
                        self.output.push_str(&format!("  %cmp{} = icmp eq i64 {}, {}\n", self.func_counter, l, r));
                        self.output.push_str(&format!("  {} = zext i1 %cmp{} to i64\n", tmp, self.func_counter));
                        return Ok(tmp);
                    }
                    _ => "add"
                };
                self.output.push_str(&format!("  {} = {} i64 {}, {}\n", tmp, op_str, l, r));
                Ok(tmp)
            }
            Expr::Call(name, args) => {
                if name == "print" {
                    // For now, print integers
                    if let Some(arg) = args.first() {
                        let val = self.compile_expr(arg)?;
                        self.output.push_str(&format!(
                            "  call i32 (i8*, ...) @printf(i8* getelementptr ([5 x i8], [5 x i8]* @.str_int, i32 0, i32 0), i64 {})\n",
                            val
                        ));
                    }
                    return Ok("0".to_string());
                }

                // Compile arguments
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.compile_expr(arg)?);
                }

                let args_str: Vec<String> = arg_vals.iter().map(|a| format!("i64 {}", a)).collect();
                self.output.push_str(&format!("  {} = call i64 @{}({})\n", tmp, name, args_str.join(", ")));
                Ok(tmp)
            }
            _ => Ok("0".to_string())
        }
    }
}

pub fn compile_to_llvm(source: &str) -> Result<String, String> {
    let mut compiler = Compiler::new();
    compiler.compile(source)
}
