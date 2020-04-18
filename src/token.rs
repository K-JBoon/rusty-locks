#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star, 

    Bang,
    BangEqual,                                
    Equal,
    EqualEqual,                              
    Greater,
    GreaterEqual,                          
    Less,
    LessEqual,                                

    Identifier,
    String,
    Number,                      

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,  
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,    

    EOF
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum LiteralValue {
    String(String),
    Number(f32),
    Null
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralValue,
    pub line: i32
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralValue, line: i32) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokenType[{}]\t\tLexeme[{}]\t\tValue[{}]", self.token_type, self.lexeme, self.literal)
    }
}
