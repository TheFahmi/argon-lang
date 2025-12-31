// Argon Lexer - Tokenizes Argon source code
// Compatible with compiler.ar v2.22.0

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn, Let, Return, If, Else, While, Print, True, False,
    Break, Continue, Struct, Enum, Match, Import, From,
    Async, Await, Extern,
    // FFI & Traits keywords
    Trait, Impl, For, SelfType,
    
    // Literals
    Number(i64),
    String(String),
    Identifier(String),
    
    // Operators
    Plus, Minus, Star, Slash, Percent,
    Eq, EqEq, NotEq, Lt, Gt, LtEq, GtEq,
    And, Or, Not,
    
    // Delimiters
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Semi, Comma, Colon, Dot, Arrow,
    
    // Attributes
    At, WasmExport, WasmImport,
    
    // Special
    Null,
    Eof,
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
        }
    }
    
    fn peek(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }
    
    fn peek_next(&self) -> Option<char> {
        self.source.get(self.pos + 1).copied()
    }
    
    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        if c == Some('\n') {
            self.line += 1;
        }
        self.pos += 1;
        c
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' && self.peek_next() == Some('/') {
                // Line comment
                while self.peek().is_some() && self.peek() != Some('\n') {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }
    
    fn read_string(&mut self) -> String {
        self.advance(); // consume opening quote
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                break;
            } else if c == '\\' {
                self.advance();
                if let Some(escaped) = self.peek() {
                    match escaped {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        'r' => s.push('\r'),
                        '\\' => s.push('\\'),
                        '"' => s.push('"'),
                        '0' => s.push('\0'),
                        _ => s.push(escaped),
                    }
                    self.advance();
                }
            } else {
                s.push(c);
                self.advance();
            }
        }
        s
    }
    
    fn read_number(&mut self) -> i64 {
        let mut num_str = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.advance();
            } else {
                break;
            }
        }
        num_str.parse().unwrap_or(0)
    }
    
    fn read_identifier(&mut self) -> String {
        let mut id = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }
        id
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        loop {
            self.skip_whitespace();
            
            let c = match self.peek() {
                Some(c) => c,
                None => {
                    tokens.push(Token::Eof);
                    break;
                }
            };
            
            let token = match c {
                '"' => Token::String(self.read_string()),
                
                '+' => { self.advance(); Token::Plus }
                '-' => { 
                    self.advance();
                    if self.peek() == Some('>') {
                        self.advance();
                        Token::Arrow
                    } else {
                        Token::Minus
                    }
                }
                '*' => { self.advance(); Token::Star }
                '/' => { self.advance(); Token::Slash }
                '%' => { self.advance(); Token::Percent }
                
                '(' => { self.advance(); Token::LParen }
                ')' => { self.advance(); Token::RParen }
                '{' => { self.advance(); Token::LBrace }
                '}' => { self.advance(); Token::RBrace }
                '[' => { self.advance(); Token::LBracket }
                ']' => { self.advance(); Token::RBracket }
                
                ';' => { self.advance(); Token::Semi }
                ',' => { self.advance(); Token::Comma }
                ':' => { self.advance(); Token::Colon }
                '.' => { self.advance(); Token::Dot }
                
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::EqEq
                    } else {
                        Token::Eq
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::NotEq
                    } else {
                        Token::Not
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::LtEq
                    } else {
                        Token::Lt
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Token::GtEq
                    } else {
                        Token::Gt
                    }
                }
                '&' => {
                    self.advance();
                    if self.peek() == Some('&') {
                        self.advance();
                    }
                    Token::And
                }
                '|' => {
                    self.advance();
                    if self.peek() == Some('|') {
                        self.advance();
                    }
                    Token::Or
                }
                
                '@' => {
                    self.advance();
                    let attr = self.read_identifier();
                    match attr.as_str() {
                        "wasm_export" => Token::WasmExport,
                        "wasm_import" => Token::WasmImport,
                        _ => Token::At,
                    }
                }
                
                _ if c.is_ascii_digit() => Token::Number(self.read_number()),
                
                _ if c.is_alphabetic() || c == '_' => {
                    let id = self.read_identifier();
                    match id.as_str() {
                        "fn" => Token::Fn,
                        "let" => Token::Let,
                        "return" => Token::Return,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "print" => Token::Print,
                        "true" => Token::True,
                        "false" => Token::False,
                        "null" => Token::Null,
                        "break" => Token::Break,
                        "continue" => Token::Continue,
                        "struct" => Token::Struct,
                        "enum" => Token::Enum,
                        "match" => Token::Match,
                        "import" => Token::Import,
                        "from" => Token::From,
                        "async" => Token::Async,
                        "await" => Token::Await,
                        "extern" => Token::Extern,
                        "trait" => Token::Trait,
                        "impl" => Token::Impl,
                        "for" => Token::For,
                        "Self" => Token::SelfType,
                        _ => Token::Identifier(id),
                    }
                }
                
                _ => {
                    self.advance();
                    continue;
                }
            };
            
            tokens.push(token);
        }
        
        tokens
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
