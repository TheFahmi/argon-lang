// Cryo Lexer - Tokenizes Cryo source code
// Compatible with compiler.ar v3.0.0

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn, Let, Return, If, Else, While, Print, True, False,
    Break, Continue, Struct, Enum, Match, Import,
    Async, Await, Extern, Defer, Macro,
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
    Semi, Comma, Colon, ColonColon, Dot, Arrow,
    
    // Attributes & Decorators
    At, WasmExport, WasmImport,
    // NestJS-style decorators
    DecController(String),  // @Controller("/path")
    DecGet(String),         // @Get("/path")
    DecPost(String),        // @Post("/path")
    DecPut(String),         // @Put("/path")
    DecDelete(String),      // @Delete("/path")
    DecPatch(String),       // @Patch("/path")
    DecInjectable,          // @Injectable()
    DecModule,              // @Module()
    DecBody,                // @Body
    DecParam(String),       // @Param("name")
    DecQuery(String),       // @Query("name")
    DecGuard(String),       // @Guard(AuthGuard)
    DecMiddleware(String),  // @Middleware(LoggerMiddleware)
    
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
            if c.is_alphanumeric() || c == '_' || c == '$' {
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
                ':' => {
                    self.advance();
                    if self.peek() == Some(':') {
                        self.advance();
                        Token::ColonColon
                    } else {
                        Token::Colon
                    }
                }
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
                    
                    // Check if it's a known decorator with optional argument
                    match attr.as_str() {
                        "wasm_export" => Token::WasmExport,
                        "wasm_import" => Token::WasmImport,
                        "Controller" | "Get" | "Post" | "Put" | "Delete" | "Patch" |
                        "Injectable" | "Module" | "Body" | "Param" | "Query" | "Guard" | "Middleware" => {
                            // Parse optional argument in parentheses for decorators
                            let arg = if self.peek() == Some('(') {
                                self.advance();
                                let mut arg_str = String::new();
                                if self.peek() == Some('"') {
                                    self.advance();
                                    while let Some(c) = self.peek() {
                                        if c == '"' { self.advance(); break; }
                                        arg_str.push(c);
                                        self.advance();
                                    }
                                } else {
                                    while let Some(c) = self.peek() {
                                        if c == ')' { break; }
                                        if !c.is_whitespace() { arg_str.push(c); }
                                        self.advance();
                                    }
                                }
                                if self.peek() == Some(')') { self.advance(); }
                                arg_str
                            } else {
                                String::new()
                            };
                            
                            match attr.as_str() {
                                "Controller" => Token::DecController(arg),
                                "Get" => Token::DecGet(arg),
                                "Post" => Token::DecPost(arg),
                                "Put" => Token::DecPut(arg),
                                "Delete" => Token::DecDelete(arg),
                                "Patch" => Token::DecPatch(arg),
                                "Injectable" => Token::DecInjectable,
                                "Module" => Token::DecModule,
                                "Body" => Token::DecBody,
                                "Param" => Token::DecParam(arg),
                                "Query" => Token::DecQuery(arg),
                                "Guard" => Token::DecGuard(arg),
                                "Middleware" => Token::DecMiddleware(arg),
                                _ => Token::At, // shouldn't happen
                            }
                        }
                        _ => {
                            // Unknown @ identifier - this is a builtin call like @sleep(1)
                            // Push Token::At, then push identifier, let parser handle the rest
                            tokens.push(Token::At);
                            Token::Identifier(attr)
                        }
                    }
                }
                
                _ if c.is_ascii_digit() => Token::Number(self.read_number()),
                
                _ if c.is_alphabetic() || c == '_' || c == '$' => {
                    let id = self.read_identifier();
                    match id.as_str() {
                        "fn" => Token::Fn,
                        "macro" => Token::Macro,
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
                        // Note: "from" is now treated as identifier, parser handles import syntax
                        "async" => Token::Async,
                        "await" => Token::Await,
                        "extern" => Token::Extern,
                        "defer" => Token::Defer,
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
