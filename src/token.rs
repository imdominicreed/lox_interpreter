use std::any::Any;

#[derive(Copy, Clone)]
enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicol, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nill, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}



pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn Any>,
    line: i32,
}

impl Token {

    pub fn to_string(&self) -> String {
        format!("{} {}", self.token_type as u16, self.lexeme)
    }
}