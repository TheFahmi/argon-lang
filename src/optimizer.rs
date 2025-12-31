// Argon AST Optimizer
// Performs primitive constant folding

use crate::parser::{Expr, Stmt, TopLevel};

pub struct Optimizer;

impl Optimizer {
    pub fn new() -> Self {
        Optimizer
    }

    pub fn optimize(&self, ast: Vec<TopLevel>) -> Vec<TopLevel> {
        ast.into_iter().map(|item| self.optimize_toplevel(item)).collect()
    }

    fn optimize_toplevel(&self, item: TopLevel) -> TopLevel {
        match item {
            TopLevel::Function(mut f) => {
                if let Some(body) = f.body {
                    f.body = Some(self.optimize_stmts(body));
                }
                TopLevel::Function(f)
            }
            TopLevel::Impl(mut impl_def) => {
                impl_def.methods = impl_def.methods.into_iter().map(|mut m| {
                    if let Some(body) = m.body {
                        m.body = Some(self.optimize_stmts(body));
                    }
                    m
                }).collect();
                TopLevel::Impl(impl_def)
            }
            TopLevel::Let(name, expr) => TopLevel::Let(name, self.optimize_expr(expr)),
            _ => item,
        }
    }

    fn optimize_stmts(&self, stmts: Vec<Stmt>) -> Vec<Stmt> {
        stmts.into_iter().map(|s| self.optimize_stmt(s)).collect()
    }

    fn optimize_stmt(&self, stmt: Stmt) -> Stmt {
        match stmt {
            Stmt::Let(name, typ, expr) => Stmt::Let(name, typ, self.optimize_expr(expr)),
            Stmt::Assign(name, expr) => Stmt::Assign(name, self.optimize_expr(expr)),
            Stmt::Expr(expr) => Stmt::Expr(self.optimize_expr(expr)),
            Stmt::Return(Some(expr)) => Stmt::Return(Some(self.optimize_expr(expr))),
            Stmt::Print(expr) => Stmt::Print(self.optimize_expr(expr)),
            Stmt::If(cond, then_block, else_block) => {
                let cond = self.optimize_expr(cond);
                let then_block = self.optimize_stmts(then_block);
                let else_block = else_block.map(|b| self.optimize_stmts(b));

                // Const if optimization
                match &cond {
                    Expr::Bool(true) => Stmt::Block(then_block),
                    Expr::Bool(false) => {
                        if let Some(else_stmts) = else_block {
                            Stmt::Block(else_stmts)
                        } else {
                            Stmt::Block(vec![]) // Empty block = no-op
                        }
                    }
                    _ => Stmt::If(cond, then_block, else_block)
                }
            }
            Stmt::While(cond, body) => {
                let cond = self.optimize_expr(cond);
                let body = self.optimize_stmts(body);
                // While false -> remove?
                 match &cond {
                    Expr::Bool(false) => Stmt::Block(vec![]),
                    _ => Stmt::While(cond, body)
                }
            }
            Stmt::Block(stmts) => Stmt::Block(self.optimize_stmts(stmts)),
            _ => stmt,
        }
    }

    fn optimize_expr(&self, expr: Expr) -> Expr {
        match expr {
            Expr::BinOp(left, op, right) => {
                let l = self.optimize_expr(*left);
                let r = self.optimize_expr(*right);

                match (l, op.as_str(), r) {
                    // Int Arithmetic
                    (Expr::Number(a), "+", Expr::Number(b)) => Expr::Number(a + b),
                    (Expr::Number(a), "-", Expr::Number(b)) => Expr::Number(a - b),
                    (Expr::Number(a), "*", Expr::Number(b)) => Expr::Number(a * b),
                    (Expr::Number(a), "/", Expr::Number(b)) => {
                        if b != 0 { Expr::Number(a / b) } else { Expr::BinOp(Box::new(Expr::Number(a)), op, Box::new(Expr::Number(b))) }
                    },
                    (Expr::Number(a), "%", Expr::Number(b)) => {
                        if b != 0 { Expr::Number(a % b) } else { Expr::BinOp(Box::new(Expr::Number(a)), op, Box::new(Expr::Number(b))) }
                    },
                    
                    // Comparison
                    (Expr::Number(a), "<", Expr::Number(b)) => Expr::Bool(a < b),
                    (Expr::Number(a), ">", Expr::Number(b)) => Expr::Bool(a > b),
                    (Expr::Number(a), "<=", Expr::Number(b)) => Expr::Bool(a <= b),
                    (Expr::Number(a), ">=", Expr::Number(b)) => Expr::Bool(a >= b),
                    (Expr::Number(a), "==", Expr::Number(b)) => Expr::Bool(a == b),
                    (Expr::Number(a), "!=", Expr::Number(b)) => Expr::Bool(a != b),

                    // Bool Logic
                    (Expr::Bool(a), "&&", Expr::Bool(b)) => Expr::Bool(a && b),
                    (Expr::Bool(a), "||", Expr::Bool(b)) => Expr::Bool(a || b),
                    (Expr::Bool(a), "==", Expr::Bool(b)) => Expr::Bool(a == b),
                    (Expr::Bool(a), "!=", Expr::Bool(b)) => Expr::Bool(a != b),

                    // Fallback
                    (l, op, r) => Expr::BinOp(Box::new(l), op.to_string(), Box::new(r)),
                }
            }
            Expr::UnaryOp(op, expr) => {
                let e = self.optimize_expr(*expr);
                match (op.as_str(), e) {
                    ("-", Expr::Number(a)) => Expr::Number(-a),
                    ("!", Expr::Bool(a)) => Expr::Bool(!a),
                    (op, e) => Expr::UnaryOp(op.to_string(), Box::new(e)),
                }
            }
            Expr::Call(name, args) => {
                let args = args.into_iter().map(|a| self.optimize_expr(a)).collect();
                Expr::Call(name, args)
            }
            Expr::MethodCall(obj, method, args) => {
                 let obj = self.optimize_expr(*obj);
                 let args = args.into_iter().map(|a| self.optimize_expr(a)).collect();
                 Expr::MethodCall(Box::new(obj), method, args)
            }
            Expr::Index(arr, idx) => {
                Expr::Index(Box::new(self.optimize_expr(*arr)), Box::new(self.optimize_expr(*idx)))
            }
            Expr::Field(obj, f) => {
                Expr::Field(Box::new(self.optimize_expr(*obj)), f)
            }
            Expr::Array(items) => {
                Expr::Array(items.into_iter().map(|e| self.optimize_expr(e)).collect())
            }
            Expr::StructInit(name, fields) => {
                let fields = fields.into_iter().map(|(k, v)| (k, self.optimize_expr(v))).collect();
                Expr::StructInit(name, fields)
            }
            // Leaf nodes
            _ => expr,
        }
    }
}
