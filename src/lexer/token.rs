use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    Identifier, Integer, HexInteger, Double, String,

    From, Import, Fun, Mut,

    BraceOpen, BraceClose, Semicolon,
    ParenOpen, ParenClose, Colon,
    Equals, Greater, GreaterEqual,
    EqualTo, Less, LessEqual, NotEqual,
    Dot, Exclamation,
    Plus, Minus, Asterisk, ForwardSlash
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("from", TokenType::From),
        ("import", TokenType::Import),
        ("fun", TokenType::Fun),
        ("mut", TokenType::Mut)
    ]);
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub value: String
}

impl Token {
    pub fn new(typ: TokenType, value: String) -> Token {
        Token { typ, value }
    }
}
