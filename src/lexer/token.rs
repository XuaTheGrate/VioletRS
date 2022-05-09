use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String), Integer(String),
    HexInteger(String), Double(String),
    String(String),

    From, Import, Fun, Mut,

    BraceOpen, BraceClose, Semicolon,
    ParenOpen, ParenClose, Colon,
    Equals, Greater, GreaterEqual,
    EqualTo, Less, LessEqual, NotEqual,
    Dot, Exclamation,
    Plus, Minus, Asterisk, ForwardSlash
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, Token> = HashMap::from([
        ("from", Token::From),
        ("import", Token::Import),
        ("fun", Token::Fun),
        ("mut", Token::Mut)
    ]);
}
