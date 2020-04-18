use crate::token::{Token, TokenType, LiteralValue};
use crate::RustyLocks;
use std::collections::HashMap;

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
    rl: &'a mut RustyLocks,
    keywords: HashMap<String, TokenType>
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, rl: &'a mut RustyLocks) -> Scanner<'a> {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();

        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);

        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            rl: rl,
            keywords: keywords 
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::from(""), LiteralValue::Null, self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => { self.add_token(TokenType::LeftParen, None); },
            ')' => { self.add_token(TokenType::RightParen, None); },
            '{' => { self.add_token(TokenType::LeftBrace, None); },
            '}' => { self.add_token(TokenType::RightBrace, None); },
            ',' => { self.add_token(TokenType::Comma, None); },
            '.' => { self.add_token(TokenType::Dot, None); },
            '-' => { self.add_token(TokenType::Minus, None); },
            '+' => { self.add_token(TokenType::Plus, None); },
            ';' => { self.add_token(TokenType::Semicolon, None); },
            '*' => { self.add_token(TokenType::Star, None); },
            '!' => {
                let tt = match self.match_char('=') {
                    true => {
                        TokenType::BangEqual
                    },
                    false => {
                        TokenType::Bang
                    }
                };
                self.add_token(tt, None);
            },
            '=' => {
                let tt = match self.match_char('=') {
                    true => {
                        TokenType::EqualEqual
                    },
                    false => {
                        TokenType::Equal
                    }
                };
                self.add_token(tt, None);
            },
            '<' => {
                let tt = match self.match_char('=') {
                    true => {
                        TokenType::LessEqual
                    },
                    false => {
                        TokenType::Less
                    }
                };
                self.add_token(tt, None);
            },
            '>' => {
                let tt = match self.match_char('=') {
                    true => {
                        TokenType::GreaterEqual
                    },
                    false => {
                        TokenType::Greater
                    }
                };
                self.add_token(tt, None);
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            },
            ' '  => { /* Ignore whitespace */ },
            '\r' => { /* Ignore whitespace */  },
            '\t' => { /* Ignore whitespace */ },
            '\n' => { self.line += 1; },
            '"' => {
                self.string();
            },
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    self.rl.error(self.line, String::from("Unexpected character"));
                }
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.rl.error(self.line, String::from("Unterminated string"));
        }

        self.advance(); // consume closing "

        let value = self.substr(1, 1);

        self.add_token(TokenType::String, Some(LiteralValue::String(value)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number, Some(LiteralValue::Number(self.substr(0, 0).parse::<f32>().expect("failed to parse f32 in number"))));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = self.substr(0, 0);

        let token_type = self.keywords.get(&text);

        if let Some(tt) = token_type {
            let token_type_clone = tt.clone();
            self.add_token(token_type_clone, None);
        } else {
            self.add_token(TokenType::Identifier, None);
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.char_at_current()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current as usize).expect("couldn't peek char");
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as i32 {
            return '\0';
        }

        self.source.chars().nth((self.current + 1) as usize).expect("couldn't peek_next char")
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            if self.source.chars().nth(self.current as usize).expect("couldn't look ahead with match_char") != expected {
                return false;
            } else {
                self.current += 1;
                return true;
            }
        }
    }

    fn char_at_current(&self) -> char {
        self.source.chars().nth((self.current - 1) as usize).expect("couldn't get character at current")
    }

    fn substr(&self, offset_start: i32, offset_end: i32) -> String {
        self.source.chars().skip((self.start + offset_start) as usize).take((self.current - (self.start + offset_start) - offset_end) as usize).collect()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text: String = self.source.chars().skip(self.start as usize).take((self.current - self.start) as usize).collect();
        let literal_value = match literal {
            Some(v) => v,
            _ => LiteralValue::Null
        };

        self.tokens.push(Token::new(token_type, text, literal_value, self.line));
    }
}
