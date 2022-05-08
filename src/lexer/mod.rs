pub mod token;

use std::{fs, process};

pub use self::token::{Token, TokenType, KEYWORDS};

pub struct Lexer {
    pub lineno: usize,
    pub lineidx: usize,
    data: Vec<String>
}

impl Lexer {
    pub fn new(data: Vec<String>) -> Lexer {
        Lexer { lineno: 0, lineidx: 0, data: data }
    }

    pub fn from_text(text: String) -> Lexer {
        let parsed = text.split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Lexer::new(parsed)
    }

    pub fn from_file(fname: &str) -> Lexer {
        let data = fs::read_to_string(fname)
            .unwrap_or_else(|err| {
                eprintln!("Error occured reading file: {}", err);
                process::exit(1);
            });
        Lexer::from_text(data)
    }

    pub fn analyze(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = match self.read_next() {
                Ok(Some(t)) => t,
                Ok(None) => break,
                Err(e) => {
                    panic!("Error occured during analyzing: {}", e);
                }
            };
            tokens.push(token);
        }

        tokens
    }

    fn current(&self) -> Option<char> {
        self.data.get(self.lineno)
            .map(|s| s.chars().nth(self.lineidx))
            .flatten()
    }

    fn advance(&mut self) -> Option<()> {
        let next = self.lineidx + 1;
        let line = &self.data[self.lineno];
        if let None = line.chars().nth(next) {
            self.lineidx = 0;
            self.lineno += 1;
            Some(())
        } else {
            self.lineidx += 1;
            None
        }
    }

    fn skip_whitespace(&mut self) -> Result<(), String> {
        // while self.advance().is_whitespace() {};
        while let Some(c) = self.current() {
            if !c.is_whitespace() { break; }
            self.advance();
        }
        Ok(())
    }

    fn keyword(&self, name: &str) -> Option<Token> {
        KEYWORDS.get(&name)
            .map(|&t| Token::new(t, name.to_string()))
    }

    fn identifier(&mut self) -> Result<Token, String> {
        let mut word = self.current().unwrap().to_string();

        self.advance();
        let mut char = match self.current() {
            Some(c) => c,
            None => return Ok(Token::new(TokenType::Identifier, word))
        };

        while char.is_alphanumeric() || char == '_' {
            word.push(char);
            self.advance();
            char = match self.current() {
                Some(c) => c,
                None => break
            };
        };

        Ok(self.keyword(&word).unwrap_or_else(|| Token::new(TokenType::Identifier, word)))
    }

    fn hexdigit(&mut self, mut num: String) -> Result<Token, String> {
        self.advance();  // consume the `x` from `0x`
        let current = self.current() // requires at least 1 digit after the x
                                .filter(|t| t.is_digit(16))
                                .ok_or("Expected hexadecimal digit")?;

        num.push(current);
        self.advance();

        while let Some(c) = self.current().filter(|t| t.is_digit(16)) {
            num.push(c);
            self.advance();
        }
        Ok(Token::new(TokenType::HexInteger, num))
    }

    fn digit(&mut self, mut num: String) -> Result<Token, String> {
        while let Some(c) = self.current().filter(|t| t.is_digit(10)) {
            num.push(c);
            self.advance();
        }

        match self.current() {
            Some(c) if c == '.' => {
                self.advance();  // consume `.`
                let next = self.current()
                    .filter(|c| c.is_digit(10))
                    .ok_or("Expected digit after `.`")?;
                num.push(next);
                self.advance();
                
                while let Some(c) = self.current().filter(|c| c.is_digit(10)) {
                    num.push(c);
                    self.advance();
                }
                Ok(Token::new(TokenType::Double, num))
            }
            _ => Ok(Token::new(TokenType::Integer, num))
        }
    }

    fn number(&mut self) -> Result<Token, String> {
        let num = self.current()
                            .unwrap()
                            .to_string();

        self.advance();
        let next = self.current().ok_or("Expected digit or `x` for hexadecimal digits")?;

        if next == 'x' { self.hexdigit(num) } else { self.digit(num) }
    }

    fn string(&mut self) -> Result<Token, String> {
        let mut str = self.current().unwrap().to_string();
        let mut escape = false;

        loop {
            self.advance();
            let next = self.current().ok_or("Unterminated string literal")?;
            str.push(next);
            match next {
                '"' => if !escape { break } else { escape = false },
                '\\' => escape = !escape,
                _ => escape = false
            }
        };
        self.advance();
        Ok(Token::new(TokenType::String, str))
    }

    fn peek_next(&self) -> Option<char> {
        let lineidx = self.lineidx + 1;
        let line = &self.data[self.lineno];
        if let Some(c) = line.chars().nth(lineidx) {
            return Some(c);
        };

        let lineno = self.lineno + 1;

        self.data.get(lineno).map(|s| s.chars().nth(0)).flatten()
    }

    fn read_next(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace()?;
        
        let c = match self.current() {
            Some(c) => c,
            None => return Ok(None)
        };

        if c.is_alphabetic() || c == '_' {
            return Ok(Some(self.identifier()?));
        }

        if c.is_numeric() {
            return Ok(Some(self.number()?));
        }

        if c == '"' {
            return Ok(Some(self.string()?));
        }

        let mut value = c.to_string();

        let token = match c {
            '{' => TokenType::BraceOpen,
            '}' => TokenType::BraceClose,
            ';' => TokenType::Semicolon,
            '(' => TokenType::ParenOpen,
            ')' => TokenType::ParenClose,
            ':' => TokenType::Colon,
            '.' => TokenType::Dot,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '/' => {
                match self.peek_next() {
                    Some('/') => {
                        self.advance();
                        loop {
                            if self.advance().is_some() { break; }
                        }

                        return self.read_next();
                    }
                    _ => TokenType::ForwardSlash
                }
            }
            '=' => {
                match self.peek_next() {
                    Some('=') => {
                        self.advance();
                        value.push('=');
                        TokenType::EqualTo
                    },
                    _ => TokenType::Equals
                }
            }
            '>' => {
                match self.peek_next() {
                    Some('=') => {
                        self.advance();
                        value.push('=');
                        TokenType::GreaterEqual
                    },
                    _ => TokenType::Greater
                }
            }
            '<' => {
                match self.peek_next() {
                    Some('=') => {
                        self.advance();
                        value.push('=');
                        TokenType::LessEqual
                    },
                    _ => TokenType::Less
                }
            }
            '!' => {
                match self.peek_next() {
                    Some('=') => {
                        self.advance();
                        value.push('=');
                        TokenType::NotEqual
                    },
                    _ => TokenType::Exclamation
                }
            }
            _ => {
                return Err(format!("Unexpected character `{}`", c));
            }
        };
        self.advance();
        Ok(Some(Token::new(token, value)))
    }
}