// Cryo Macro Expander
// Performs AST transformation (Macro Expansion)

use crate::parser::{Expr, Stmt, TopLevel, MacroDef};
use std::collections::HashMap;

pub struct Expander {
    macros: HashMap<String, MacroDef>,
}

impl Expander {
    pub fn new() -> Self {
        Expander { macros: HashMap::new() }
    }

    pub fn expand(&mut self, ast: Vec<TopLevel>) -> Vec<TopLevel> {
        // 1. Collect macros
        let mut remaining_ast = Vec::new();
        for item in ast {
            if let TopLevel::Macro(def) = item {
                self.macros.insert(def.name.clone(), def);
            } else {
                remaining_ast.push(item);
            }
        }

        // 2. Expand
        remaining_ast.into_iter().map(|item| self.expand_toplevel(item)).collect()
    }

    fn expand_toplevel(&self, item: TopLevel) -> TopLevel {
        match item {
            TopLevel::Function(mut f) => {
                if let Some(body) = f.body {
                    f.body = Some(self.expand_stmts(body));
                }
                TopLevel::Function(f)
            }
            TopLevel::Impl(mut impl_def) => {
                impl_def.methods = impl_def.methods.into_iter().map(|mut m| {
                    if let Some(body) = m.body {
                        m.body = Some(self.expand_stmts(body));
                    }
                    m
                }).collect();
                TopLevel::Impl(impl_def)
            }
            _ => item, // Structure/Enum defs don't have code to expand
        }
    }

    fn expand_stmts(&self, stmts: Vec<Stmt>) -> Vec<Stmt> {
        stmts.into_iter().map(|s| self.expand_stmt(s)).collect()
    }

    fn expand_stmt(&self, stmt: Stmt) -> Stmt {
        match stmt {
            Stmt::Expr(Expr::Call(name, args)) => {
                // Check if macro
                if let Some(def) = self.macros.get(&name) {
                    if args.len() == def.params.len() {
                        // Bindings
                        let mut bindings = HashMap::new();
                        for (i, param) in def.params.iter().enumerate() {
                            // Expand arguments before binding? Yes.
                            let arg = self.expand_expr(args[i].clone());
                            bindings.insert(param.clone(), arg);
                        }
                        
                        // Instantiate body
                        let expanded_body = self.instantiate_stmts(&def.body, &bindings);
                        return Stmt::Block(expanded_body);
                    }
                }
                // Not a macro or arg mismatch (silent failure/runtime error)
                // Just recurse args
                let args = args.into_iter().map(|a| self.expand_expr(a)).collect();
                Stmt::Expr(Expr::Call(name, args))
            }
            // Recurse other stmts
            Stmt::Block(stmts) => Stmt::Block(self.expand_stmts(stmts)),
            Stmt::If(cond, then_b, else_b) => Stmt::If(self.expand_expr(cond), self.expand_stmts(then_b), else_b.map(|b| self.expand_stmts(b))),
            Stmt::While(cond, body) => Stmt::While(self.expand_expr(cond), self.expand_stmts(body)),
            Stmt::Let(n, t, e) => Stmt::Let(n, t, self.expand_expr(e)),
            Stmt::Assign(n, e) => Stmt::Assign(n, self.expand_expr(e)),
            Stmt::Return(Some(e)) => Stmt::Return(Some(self.expand_expr(e))),
            Stmt::Print(e) => Stmt::Print(self.expand_expr(e)),
            Stmt::Defer(s) => Stmt::Defer(Box::new(self.expand_stmt(*s))),
            _ => stmt 
        }
    }

    fn expand_expr(&self, expr: Expr) -> Expr {
        // Expressions usually don't contain macro calls that return Blocks.
        // But we should recurse.
        match expr {
            Expr::UnaryOp(op, e) => Expr::UnaryOp(op, Box::new(self.expand_expr(*e))),
            Expr::BinOp(l, op, r) => Expr::BinOp(Box::new(self.expand_expr(*l)), op, Box::new(self.expand_expr(*r))),
            Expr::Call(n, args) => Expr::Call(n, args.into_iter().map(|a| self.expand_expr(a)).collect()),
            // ...
            _ => expr
        }
    }

    fn instantiate_stmts(&self, stmts: &[Stmt], bindings: &HashMap<String, Expr>) -> Vec<Stmt> {
        stmts.iter().map(|s| self.instantiate_stmt(s, bindings)).collect()
    }

    fn instantiate_stmt(&self, stmt: &Stmt, bindings: &HashMap<String, Expr>) -> Stmt {
        // Recursive instantiation with substitution
        match stmt {
            Stmt::Expr(e) => Stmt::Expr(self.instantiate_expr(e, bindings)),
            Stmt::Print(e) => Stmt::Print(self.instantiate_expr(e, bindings)),
            Stmt::Let(n, t, e) => Stmt::Let(n.clone(), t.clone(), self.instantiate_expr(e, bindings)),
            Stmt::Assign(n, e) => Stmt::Assign(n.clone(), self.instantiate_expr(e, bindings)),
            Stmt::If(c, t, e) => Stmt::If(self.instantiate_expr(c, bindings), self.instantiate_stmts(t, bindings), e.as_ref().map(|b| self.instantiate_stmts(b, bindings))),
            // ...
            _ => stmt.clone() // Fallback clone if deep logic missing
        }
    }

    fn instantiate_expr(&self, expr: &Expr, bindings: &HashMap<String, Expr>) -> Expr {
        match expr {
            Expr::Identifier(name) if name.starts_with('$') => {
                 let key = &name[1..];
                 if let Some(val) = bindings.get(key) {
                     val.clone()
                 } else {
                     Expr::Identifier(name.clone())
                 }
            }
            Expr::UnaryOp(op, e) => Expr::UnaryOp(op.clone(), Box::new(self.instantiate_expr(e, bindings))),
            Expr::BinOp(l, op, r) => Expr::BinOp(Box::new(self.instantiate_expr(l, bindings)), op.clone(), Box::new(self.instantiate_expr(r, bindings))),
            Expr::Call(n, args) => Expr::Call(n.clone(), args.iter().map(|a| self.instantiate_expr(a, bindings)).collect()),
            Expr::MethodCall(obj, m, args) => Expr::MethodCall(Box::new(self.instantiate_expr(obj, bindings)), m.clone(), args.iter().map(|a| self.instantiate_expr(a, bindings)).collect()),
            Expr::Field(obj, f) => Expr::Field(Box::new(self.instantiate_expr(obj, bindings)), f.clone()),
            Expr::Index(arr, idx) => Expr::Index(Box::new(self.instantiate_expr(arr, bindings)), Box::new(self.instantiate_expr(idx, bindings))),
            Expr::Array(items) => Expr::Array(items.iter().map(|e| self.instantiate_expr(e, bindings)).collect()),
            Expr::StructInit(name, fields) => Expr::StructInit(name.clone(), fields.iter().map(|(k,v)| (k.clone(), self.instantiate_expr(v, bindings))).collect()),
            _ => expr.clone()
        }
    }
}
