// Argon Parser - Parses tokens into AST
// Compatible with compiler.ar v3.0.0

#![allow(dead_code)]

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    String(String),
    Bool(bool),
    Null,
    Identifier(String),
    BinOp(Box<Expr>, String, Box<Expr>),
    UnaryOp(String, Box<Expr>),
    Call(String, Vec<Expr>),
    MethodCall(Box<Expr>, String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Field(Box<Expr>, String),
    Array(Vec<Expr>),
    StructInit(String, Vec<(String, Expr)>),
    Await(Box<Expr>),
    StaticMethodCall(String, String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Option<String>, Expr),
    Assign(String, Expr),
    IndexAssign(Expr, Expr, Expr),
    FieldAssign(Expr, String, Expr),
    Return(Option<Expr>),
    Print(Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    Break,
    Continue,
    Expr(Expr),
    Block(Vec<Stmt>),
    Defer(Box<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub typ: Option<String>,
}

/// Decorator for NestJS-style annotations
#[derive(Debug, Clone)]
pub struct Decorator {
    pub name: String,
    pub arg: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Option<Vec<Stmt>>, // Body is optional for traits/extern
    pub is_async: bool,
    pub return_type: Option<String>,
    pub decorators: Vec<Decorator>, // @Get, @Post, etc.
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub decorators: Vec<Decorator>, // @Controller, @Injectable, etc.
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: String,
    pub methods: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct ImplDef {
    pub trait_name: String,
    pub type_name: String,
    pub methods: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct ExternBlock {
    pub abi: String,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct MacroDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Function(Function),
    Struct(StructDef),
    Enum(EnumDef),
    Let(String, Expr),
    Import(String, Vec<String>),
    Trait(TraitDef),
    Impl(ImplDef),
    Extern(ExternBlock),
    Macro(MacroDef),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }
    
    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        self.pos += 1;
        tok
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.peek() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.peek()))
        }
    }
    
    fn match_token(&mut self, expected: &Token) -> bool {
        if self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn collect_decorators(&mut self) -> Vec<Decorator> {
        let mut decorators = Vec::new();
        loop {
            let token = self.peek().clone();
            let decorator_opt = match token {
                Token::DecController(arg) => Some(("Controller", arg)),
                Token::DecGet(arg) => Some(("Get", arg)),
                Token::DecPost(arg) => Some(("Post", arg)),
                Token::DecPut(arg) => Some(("Put", arg)),
                Token::DecDelete(arg) => Some(("Delete", arg)),
                Token::DecPatch(arg) => Some(("Patch", arg)),
                Token::DecInjectable => Some(("Injectable", "".to_string())),
                Token::DecModule => Some(("Module", "".to_string())),
                Token::DecBody => Some(("Body", "".to_string())),
                Token::DecParam(arg) => Some(("Param", arg)),
                Token::DecQuery(arg) => Some(("Query", arg)),
                Token::DecGuard(arg) => Some(("Guard", arg)),
                Token::DecMiddleware(arg) => Some(("Middleware", arg)),
                
                Token::At | Token::WasmExport | Token::WasmImport => {
                    self.advance();
                    if self.peek() == &Token::LParen {
                        self.advance();
                        while self.peek() != &Token::RParen && self.peek() != &Token::Eof {
                            self.advance();
                        }
                        self.advance();
                    }
                    None
                }
                _ => break,
            };

            if let Some((name, arg)) = decorator_opt {
                decorators.push(Decorator { name: name.to_string(), arg });
                self.advance();
            }
        }
        decorators
    }
    
    pub fn parse(&mut self) -> Result<Vec<TopLevel>, String> {
        let mut items = Vec::new();
        
        while self.peek() != &Token::Eof {
            let decorators = self.collect_decorators();
            
            match self.peek() {
                Token::Fn | Token::Async => {
                    items.push(TopLevel::Function(self.parse_function_with_decorators(decorators)?));
                }
                Token::Struct => {
                    items.push(TopLevel::Struct(self.parse_struct_with_decorators(decorators)?));
                }
                Token::Enum => {
                    items.push(TopLevel::Enum(self.parse_enum()?));
                }
                // ... rest of match arms need to be preserved/modified if decorators apply
                Token::Let => {
                    let (name, expr) = self.parse_global_let()?;
                    items.push(TopLevel::Let(name, expr));
                }
                Token::Import => {
                    let (path, names) = self.parse_import()?;
                    items.push(TopLevel::Import(path, names));
                }
                Token::Extern => {
                    items.push(TopLevel::Extern(self.parse_extern()?));
                }
                Token::Trait => {
                    items.push(TopLevel::Trait(self.parse_trait()?));
                }
                Token::Impl => {
                    items.push(TopLevel::Impl(self.parse_impl()?));
                }
                Token::Macro => {
                    items.push(TopLevel::Macro(self.parse_macro()?));
                }
                _ => return Err(format!("Unexpected token at top level: {:?}", self.peek())),
            }
        }

        
        Ok(items)
    }

    fn parse_macro(&mut self) -> Result<MacroDef, String> {
        self.expect(Token::Macro)?;
        let name = match self.advance() {
            Token::Identifier(s) => s,
            t => return Err(format!("Expected macro name, got {:?}", t)),
        };
        
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        if self.peek() != &Token::RParen {
            loop {
                match self.advance() {
                    Token::Identifier(s) => params.push(s),
                    t => return Err(format!("Expected parameter name, got {:?}", t)),
                }
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.expect(Token::RParen)?;
        
        if self.peek() != &Token::LBrace {
             return Err("Expected block for macro body".to_string());
        }
        let body = self.parse_block()?;
        Ok(MacroDef { name, params, body })
    }
    
    fn parse_function(&mut self) -> Result<Function, String> {
        self.parse_function_with_decorators(vec![])
    }
    
    fn parse_function_with_decorators(&mut self, decorators: Vec<Decorator>) -> Result<Function, String> {
        let is_async = self.match_token(&Token::Async);
        self.expect(Token::Fn)?;
        
        let name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected function name".to_string()),
        };
        
        // Skip generic params <T>
        if self.peek() == &Token::Lt {
            self.advance();
            while self.peek() != &Token::Gt && self.peek() != &Token::Eof {
                self.advance();
            }
            self.advance();
        }
        
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        while self.peek() != &Token::RParen {
            // Parse parameter name - allow SelfType as well
            let pname = match self.advance() {
                Token::Identifier(s) => s,
                Token::SelfType => "self".to_string(),
                _t => {
                    // Return the token so we exit cleanly
                    self.pos -= 1; // Put token back
                    break;
                }
            };
            let mut ptype = None;
            if self.match_token(&Token::Colon) {
                ptype = Some(self.parse_type()?);
            }
            params.push(Param { name: pname, typ: ptype });
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        self.expect(Token::RParen)?;
        
        // Return type
        let mut return_type = None;
        if self.match_token(&Token::Arrow) {
            return_type = Some(self.parse_type()?);
        }
        
        let body = if self.match_token(&Token::Semi) {
            None
        } else {
            Some(self.parse_block()?)
        };
        
        Ok(Function {
            name,
            params,
            body,
            is_async,
            return_type,
            decorators,
        })
    }
    
    fn parse_type(&mut self) -> Result<String, String> {
        // Handle pointer types *T
        if self.match_token(&Token::Star) {
            let inner = self.parse_type()?;
            return Ok(format!("*{}", inner));
        }
        
        // Handle Self type
        if self.match_token(&Token::SelfType) {
            return Ok("Self".to_string());
        }
        
        // Handle array types [T]
        if self.match_token(&Token::LBracket) {
            let inner = self.parse_type()?;
            self.expect(Token::RBracket)?;
            return Ok(format!("[{}]", inner));
        }
        
        let mut typ = match self.advance() {
            Token::Identifier(s) => s,
            t => return Err(format!("Expected type, got {:?}", t)),
        };
        
        // Handle generic types like Box<T>
        if self.peek() == &Token::Lt {
            typ.push('<');
            self.advance();
            while self.peek() != &Token::Gt && self.peek() != &Token::Eof {
                match self.advance() {
                    Token::Identifier(s) => typ.push_str(&s),
                    Token::Comma => typ.push(','),
                    _ => {}
                }
            }
            self.advance();
            typ.push('>');
        }
        Ok(typ)
    }
    
    fn parse_trait(&mut self) -> Result<TraitDef, String> {
        self.expect(Token::Trait)?;
        let name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected trait name".to_string()),
        };
        
        self.expect(Token::LBrace)?;
        let mut methods = Vec::new();
        while self.peek() != &Token::RBrace {
            methods.push(self.parse_function()?);
        }
        self.expect(Token::RBrace)?;
        
        Ok(TraitDef { name, methods })
    }
    
    fn parse_impl(&mut self) -> Result<ImplDef, String> {
        self.expect(Token::Impl)?;
        let name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected identifier".to_string()),
        };
        
        let mut trait_name = String::new();
        let mut type_name = name; 
        
        if self.match_token(&Token::For) {
            trait_name = type_name;
            type_name = match self.advance() {
                Token::Identifier(s) => s,
                _ => return Err("Expected type name".to_string()),
            };
        }
        
        self.expect(Token::LBrace)?;
        let mut methods = Vec::new();
        while self.peek() != &Token::RBrace {
            let decorators = self.collect_decorators();
            methods.push(self.parse_function_with_decorators(decorators)?);
        }
        self.expect(Token::RBrace)?;
        
        Ok(ImplDef { trait_name, type_name, methods })
    }
    
    fn parse_extern(&mut self) -> Result<ExternBlock, String> {
        self.expect(Token::Extern)?;
        let abi = match self.peek() {
            Token::String(s) => {
                let abi = s.clone();
                self.advance();
                abi
            },
            _ => "C".to_string(),
        };
        
        // Handle single function declaration: extern "C" fn foo();
        if self.peek() == &Token::Fn {
            let func = self.parse_function()?;
            return Ok(ExternBlock { abi, functions: vec![func] });
        }
        
        // Handle block: extern "C" { ... }
        if self.match_token(&Token::LBrace) {
             let mut functions = Vec::new();
             while self.peek() != &Token::RBrace {
                 functions.push(self.parse_function()?);
             }
             self.expect(Token::RBrace)?;
             return Ok(ExternBlock { abi, functions });
        }
        
        Err("Expected fn or block after extern".to_string())
    }
    fn parse_struct(&mut self) -> Result<StructDef, String> {
        self.parse_struct_with_decorators(vec![])
    }

    fn parse_struct_with_decorators(&mut self, decorators: Vec<Decorator>) -> Result<StructDef, String> {
        self.expect(Token::Struct)?;
        let name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected struct name".to_string()),
        };
        
        // Skip generic params
        if self.peek() == &Token::Lt {
            self.advance();
            while self.peek() != &Token::Gt && self.peek() != &Token::Eof {
                self.advance();
            }
            self.advance();
        }
        
        self.expect(Token::LBrace)?;
        let mut fields = Vec::new();
        while self.peek() != &Token::RBrace {
            let fname = match self.advance() {
                Token::Identifier(s) => s,
                _ => break,
            };
            self.expect(Token::Colon)?;
            let ftype = self.parse_type()?;
            fields.push((fname, ftype));
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        self.expect(Token::RBrace)?;
        
        Ok(StructDef { name, fields, decorators })
    }
    
    fn parse_enum(&mut self) -> Result<EnumDef, String> {
        self.expect(Token::Enum)?;
        let name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected enum name".to_string()),
        };
        
        self.expect(Token::LBrace)?;
        let mut variants = Vec::new();
        while self.peek() != &Token::RBrace {
            match self.advance() {
                Token::Identifier(s) => variants.push(s),
                _ => break,
            }
            self.match_token(&Token::Comma);
        }
        self.expect(Token::RBrace)?;
        
        Ok(EnumDef { name, variants })
    }
    
    fn parse_global_let(&mut self) -> Result<(String, Expr), String> {
        self.expect(Token::Let)?;
        let name = match self.advance() {
            Token::Identifier(s) => s,
            _ => return Err("Expected variable name".to_string()),
        };
        self.expect(Token::Eq)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semi)?;
        Ok((name, expr))
    }
    
    fn parse_import(&mut self) -> Result<(String, Vec<String>), String> {
        self.expect(Token::Import)?;
        let mut names = Vec::new();
        
        // import { a, b } from "path" or import "path"
        if self.peek() == &Token::LBrace {
            self.advance();
            while self.peek() != &Token::RBrace {
                match self.advance() {
                    Token::Identifier(s) => names.push(s),
                    _ => {}
                }
                self.match_token(&Token::Comma);
            }
            self.advance();
            // Expect "from" keyword (now as identifier)
            match self.peek() {
                Token::Identifier(s) if s == "from" => { self.advance(); }
                _ => return Err("Expected 'from' after import block".to_string()),
            }
        }
        
        let path = match self.advance() {
            Token::String(s) => s,
            _ => return Err("Expected import path".to_string()),
        };
        self.match_token(&Token::Semi);
        
        Ok((path, names))
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(Token::LBrace)?;
        let mut stmts = Vec::new();
        while self.peek() != &Token::RBrace && self.peek() != &Token::Eof {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(stmts)
    }
    
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek().clone() {
            Token::Let => {
                self.advance();
                let name = match self.advance() {
                    Token::Identifier(s) => s,
                    _ => return Err("Expected variable name".to_string()),
                };
                let mut typ = None;
                if self.match_token(&Token::Colon) {
                    typ = Some(self.parse_type()?);
                }
                self.expect(Token::Eq)?;
                let expr = self.parse_expr()?;
                self.expect(Token::Semi)?;
                Ok(Stmt::Let(name, typ, expr))
            }
            Token::Return => {
                self.advance();
                let expr = if self.peek() != &Token::Semi {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                self.expect(Token::Semi)?;
                Ok(Stmt::Return(expr))
            }
            Token::Print => {
                self.advance();
                self.expect(Token::LParen)?;
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                self.expect(Token::Semi)?;
                Ok(Stmt::Print(expr))
            }
            Token::If => {
                self.advance();
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;
                let then_block = self.parse_block()?;
                let else_block = if self.match_token(&Token::Else) {
                    if self.peek() == &Token::If {
                        Some(vec![self.parse_stmt()?])
                    } else {
                        Some(self.parse_block()?)
                    }
                } else {
                    None
                };
                Ok(Stmt::If(cond, then_block, else_block))
            }
            Token::While => {
                self.advance();
                self.expect(Token::LParen)?;
                let cond = self.parse_expr()?;
                self.expect(Token::RParen)?;
                let body = self.parse_block()?;
                Ok(Stmt::While(cond, body))
            }
            Token::Break => {
                self.advance();
                self.match_token(&Token::Semi);
                Ok(Stmt::Break)
            }
            Token::Continue => {
                self.advance();
                self.match_token(&Token::Semi);
                Ok(Stmt::Continue)
            }
            Token::LBrace => {
                let stmts = self.parse_block()?;
                Ok(Stmt::Block(stmts))
            }
            Token::Defer => {
                self.advance();
                let stmt = self.parse_stmt()?;
                Ok(Stmt::Defer(Box::new(stmt)))
            }
            Token::Identifier(name) => {
                self.advance();
                if self.match_token(&Token::Eq) {
                    let expr = self.parse_expr()?;
                    self.expect(Token::Semi)?;
                    Ok(Stmt::Assign(name, expr))
                } else {
                    // Could be function call or other expression
                    self.pos -= 1; // Go back
                    let expr = self.parse_expr()?;
                    
                    if self.match_token(&Token::Eq) {
                         let val = self.parse_expr()?;
                         self.expect(Token::Semi)?;
                         match expr {
                             Expr::Field(obj, field) => Ok(Stmt::FieldAssign(*obj, field, val)),
                             Expr::Index(arr, idx) => Ok(Stmt::IndexAssign(*arr, *idx, val)),
                             _ => Err(format!("Invalid assignment target: {:?}", expr)),
                         }
                    } else {
                        self.expect(Token::Semi)?;
                        Ok(Stmt::Expr(expr))
                    }
                }
            }
            _ => {
                let expr = self.parse_expr()?;
                self.match_token(&Token::Semi);
                Ok(Stmt::Expr(expr))
            }
        }
    }
    
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.peek() == &Token::Or {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinOp(Box::new(left), "||".to_string(), Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while self.peek() == &Token::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::BinOp(Box::new(left), "&&".to_string(), Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = match self.peek() {
                Token::EqEq => "==",
                Token::NotEq => "!=",
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinOp(Box::new(left), op.to_string(), Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                Token::Lt => "<",
                Token::Gt => ">",
                Token::LtEq => "<=",
                Token::GtEq => ">=",
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::BinOp(Box::new(left), op.to_string(), Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.peek() {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::BinOp(Box::new(left), op.to_string(), Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                Token::Star => "*",
                Token::Slash => "/",
                Token::Percent => "%",
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinOp(Box::new(left), op.to_string(), Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp("!".to_string(), Box::new(expr)))
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp("-".to_string(), Box::new(expr)))
            }
            Token::Await => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Await(Box::new(expr)))
            }
            _ => self.parse_postfix(),
        }
    }
    
    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        
        loop {
            match self.peek() {
                Token::LParen => {
                    // Function call
                    if let Expr::Identifier(name) = expr.clone() {
                        self.advance();
                        let args = self.parse_args()?;
                        self.expect(Token::RParen)?;
                        expr = Expr::Call(name, args);
                    } else {
                        break;
                    }
                }
                Token::LBracket => {
                    // Index access
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(Token::RBracket)?;
                    expr = Expr::Index(Box::new(expr), Box::new(index));
                }
                Token::Dot => {
                    // Field access or method call
                    self.advance();
                    let field = match self.advance() {
                        Token::Identifier(s) => s,
                        _ => return Err("Expected field name".to_string()),
                    };
                    if self.peek() == &Token::LParen {
                        self.advance();
                        let args = self.parse_args()?;
                        self.expect(Token::RParen)?;
                        expr = Expr::MethodCall(Box::new(expr), field, args);
                    } else {
                        expr = Expr::Field(Box::new(expr), field);
                    }
                }
                Token::ColonColon => {
                     // Static method call: Type::Method()
                     if let Expr::Identifier(type_name) = expr {
                         self.advance(); // ::
                         let method_name = match self.advance() {
                             Token::Identifier(s) => s,
                             _ => return Err("Expected static method name".to_string()),
                         };
                         
                         self.expect(Token::LParen)?;
                         let args = self.parse_args()?;
                         self.expect(Token::RParen)?;
                         
                         expr = Expr::StaticMethodCall(type_name, method_name, args);
                     } else {
                         return Err("Expected identifier before ::".to_string());
                     }
                }
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        while self.peek() != &Token::RParen {
            args.push(self.parse_expr()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        Ok(args)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            Token::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Token::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Token::Null => {
                self.advance();
                Ok(Expr::Null)
            }
            Token::Identifier(name) => {
                self.advance();
                // Check for struct init: Name { field: value }
                if self.peek() == &Token::LBrace {
                    // Could be struct init - peek ahead
                    let saved_pos = self.pos;
                    self.advance();
                    
                    if self.peek() == &Token::RBrace {
                        self.advance(); // Consume RBrace
                        return Ok(Expr::StructInit(name, Vec::new()));
                    }
                    
                    if let Token::Identifier(_) = self.peek() {
                        let next_pos = self.pos + 1;
                        if self.tokens.get(next_pos) == Some(&Token::Colon) {
                            // Struct init
                            let mut fields = Vec::new();
                            while self.peek() != &Token::RBrace {
                                let fname = match self.advance() {
                                    Token::Identifier(s) => s,
                                    _ => break,
                                };
                                self.expect(Token::Colon)?;
                                let fexpr = self.parse_expr()?;
                                fields.push((fname, fexpr));
                                self.match_token(&Token::Comma);
                            }
                            self.expect(Token::RBrace)?;
                            return Ok(Expr::StructInit(name, fields));
                        }
                    }
                    self.pos = saved_pos;
                }
                Ok(Expr::Identifier(name))
            }
            Token::LBracket => {
                // Array literal
                self.advance();
                let mut elements = Vec::new();
                while self.peek() != &Token::RBracket {
                    elements.push(self.parse_expr()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
                self.expect(Token::RBracket)?;
                Ok(Expr::Array(elements))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            Token::At => {
                // Built-in function call: @name(args)
                self.advance();
                let name = match self.advance() {
                    Token::Identifier(s) => s,
                    t => return Err(format!("Expected identifier after @, got {:?}", t)),
                };
                self.expect(Token::LParen)?;
                let args = self.parse_args()?;
                self.expect(Token::RParen)?;
                Ok(Expr::Call(name, args))
            }
            _ => {
                Err(format!("Unexpected token: {:?}", self.peek()))
            }
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Vec<TopLevel>, String> {
    let mut parser = Parser::new(tokens.to_vec());
    parser.parse()
}
