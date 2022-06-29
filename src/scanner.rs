use crate::token::Token;
use std::vec::Vec;


pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    
    fn is_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    fn scan_token(&self) {
        panic!("Not implemented yet!");
    }
    
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }


        &self.tokens
    }
}
