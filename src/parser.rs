use crate::token::{Token, TokenType};
use crate::expressions::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: usize,
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current_token: 0
        }
    }

    fn expressions(&self) -> Expr {
        self.equality()
    }
    
    fn equality(&self) -> Expr {
        let expr = self.factor();
        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let right = self.factor();
            let expr = Expr::Binary {
                left: Box::new(expr),
                operator: self.previous(),
                right: Box::new(right)
            };
        }
        expr
    }

    fn comparison(&self) -> Expr {
        let expr = self.term();
        while self.matches(&[
            TokenType::Less, TokenType::Less,
            TokenType::Greater, TokenType::GreaterEqual,
            ]) {
            let right = self.term();
            let expr = Expr::Binary {
                left: Box::new(expr),
                operator: self.previous(),
                right: Box::new(right)
            };
        }
        expr
    }

    fn term(&self) -> Expr {
        let expr = self.factor();
        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let right = self.factor();
            let expr = Expr::Binary {
                left: Box::new(expr),
                operator: self.previous(),
                right: Box::new(right)
            };
        }
        expr
    }

    fn factor(&self) -> Expr {
        let expr = self.unary();
        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let right = self.unary();
            let expr = Expr::Binary {
                left: Box::new(expr),
                operator: self.previous(),
                right: Box::new(right)
            };
        }
        expr
    }

    fn unary(&self) -> Expr {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = Some(Box::new(self.unary()));
            return Expr::Unary {
                left: None,
                operator,
                right,
            }
        }
        return self.primary();
    }

    fn primary(&self) -> Expr {
        match self.previous().token_type {
            TokenType::True => Expr::Literal{value:Box::new(true)},
            TokenType::False => Expr::Literal{value:Box::new(false)},
            TokenType::Nil =>  Expr::Literal{value:Box::new(None)},
            TokenType::Number | TokenType::String => Expr::Literal{value:Box::new(self.previous().literal.unwrap())},
            TokenType::LeftParen => {
                let expr = self.expressions();
                self.matches(&[TokenType::RightParen]);
                expr
            },
            _ => panic!("this shoudl not panic but too lazy to implement!"),
        }
    }

    fn matches(&self, types : &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&self) -> Token {
        if !self.is_end() {
            self.current_token += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current_token-1]
    }
    fn check(&self, t : TokenType) -> bool {
        self.is_end() || t ==self.peek()
    }

    fn is_end(&self) -> bool {
        self.peek() == TokenType::Eof
    }

    fn peek(&self) -> TokenType {
        return self.tokens[self.current_token].token_type
    }
}