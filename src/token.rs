use std::any::Any;

#[derive(Copy, Clone,Debug)]
pub enum TokenType {
    // Single-character tokens.
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nill,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String)
}





pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_string(&self) -> String {
        match &self.literal {
            None => format!("{:?} {}", self.token_type, self.lexeme),
            Some(l) => {
                format!("{:?} {} {:?}", self.token_type, self.lexeme,l)
            }
        }
    }
}
