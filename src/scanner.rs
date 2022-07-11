use crate::token::{Token, TokenType, LiteralObject};
use std::vec::Vec;
use std::collections::HashMap;
use std::array::IntoIter;
use crate::error;


pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    identifiers: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let identifiers = HashMap::<_, _>::from_iter(IntoIter::new([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class), 
            ("else".to_string(), TokenType::Else), 
            ("false".to_string(), TokenType::False), 
            ("fun".to_string(), TokenType::Fun), 
            ("for".to_string(), TokenType::For), 
            ("if".to_string(), TokenType::If), 
            ("nil".to_string(), TokenType::Nil), 
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return), 
            ("super".to_string(), TokenType::Super), 
            ("this".to_string(), TokenType::This), 
            ("true".to_string(), TokenType::True), 
            ("var".to_string(), TokenType::Var), 
            ("while".to_string(), TokenType::While), 
            ]));
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            identifiers
        }
    }
    
    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current-1]
    }

    fn matching(&mut self, c: char) -> bool {
        if self.is_end() || c != self.source[self.current] {
            return false
        }
        self.current += 1;
        true
    }

    fn scan_next_letter(&mut self, matching_char: char, matches:TokenType, no_matches:TokenType) {
        if self.matching(matching_char) {
            self.add_token(matches, None);
        } else {
            self.add_token(no_matches, None);
        }
    }
    fn peek(&self) -> char {
        if self.is_end() {
            return '\0'
        }
        return self.source[self.current];
    }

    fn scan_token(&mut self) {
        let letter = self.advance();
        match letter {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            '*' => self.add_token(TokenType::Star, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '!' => self.scan_next_letter('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.scan_next_letter('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.scan_next_letter('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.scan_next_letter('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => { 
                if self.matching('/') {
                    while self.peek() !=  '\n' && !self.is_end() {
                        self.advance();
                    }   
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            },
            ' '|'\r'|'t' => {},
            '\n' => self.line += 1,
            '"' => self.scan_string(),
            _ => {
                if letter.is_numeric() {
                    self.scan_digit();
                } else if letter.is_alphabetic() {
                    self.scan_identifier();
                } else {
                    error::error(self.line, format!("Unexpected Character {}", letter));
                    return;
                }
            } 
        }

    }

    fn scan_digit(&mut self) {
        let mut number: f64 = self.previous().to_digit(10).unwrap() as f64;
        while self.peek().is_numeric() {
            number *= 10.0;
            number += self.peek().to_digit(10).unwrap() as f64;
            self.advance();
        }
        if !self.matching('.'){
            self.add_token(TokenType::Number, Some(LiteralObject::Number(number)));
            return
        }
        let mut power = 0.1;
        while self.peek().is_numeric() {
            number += self.peek().to_digit(10).unwrap() as f64 * power;
            power /= 10.0;
            self.advance();
        }
        self.add_token(TokenType::Number, Some(LiteralObject::Number(number)));
    }
    fn previous(&self) -> char{
        self.source[self.current-1]
    }
    fn scan_identifier(&mut self) {
        let mut string = self.previous().to_string();
        while self.peek().is_alphanumeric() {
            string.push(self.advance());
        }
        let mut token_type = TokenType::Identifier;
        match self.identifiers.get(&string) {
            Some(tt) => token_type = *tt,
            None =>{}
        }
        self.add_token(token_type, Some(LiteralObject::String(string)))
    }
    
    fn scan_string(&mut self) {
        let mut string = self.previous().to_string();
        while !self.is_end() && self.peek() != '"' {
            string.push(self.advance());
        }
        if self.is_end() {
            error::error(self.line, "Unterminating String :(".to_string());
        }

        self.add_token(TokenType::String, Some(LiteralObject::String(string)));
        self.advance();
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralObject>) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start..self.current].iter().collect(),
            literal,
            self.line
        ))
    }
    
    
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        &self.tokens
    }
}
