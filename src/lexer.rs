use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek_char() {
            match ch {
                c if c.is_whitespace() => {
                    self.next_char();
                }

                c if c.is_ascii_alphabetic() || c == '_' => {
                    tokens.push(self.lex_identifier());
                }

                c if c.is_ascii_digit() => {
                    tokens.push(self.lex_number());
                }

                '"' | '\'' => {
                    let quote = self.next_char().unwrap();
                    tokens.push(self.lex_string(quote));
                }

                '+' => {
                    self.next_char();
                    tokens.push(Token::Plus);
                }

                '-' => {
                    self.next_char();
                    tokens.push(Token::Minus);
                }

                '*' => {
                    self.next_char();
                    tokens.push(Token::Star);
                }

                '/' => {
                    self.next_char();
                    if self.peek_char() == Some('/') {
                        self.skip_line_comment();
                    } else {
                        tokens.push(Token::Slash);
                    }
                }

                '=' => {
                    self.next_char();
                    tokens.push(Token::Equal);
                }

                '(' => {
                    self.next_char();
                    tokens.push(Token::LParen);
                }

                ')' => {
                    self.next_char();
                    tokens.push(Token::RParen);
                }

                ',' => {
                    self.next_char();
                    tokens.push(Token::Comma);
                }

                ';' => {
                    self.next_char();
                    tokens.push(Token::Semicolon);
                }

                _ => {
                    println!("Lexer warning: unknown character '{}'", ch);
                    self.next_char();
                }
            }
        }

        tokens.push(Token::Eof);
        tokens
    }

    fn lex_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        // Bisa tambahkan keyword check di sini jika nanti mendukung if/while/etc
        Token::Identifier(ident)
    }

    fn lex_number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() || c == '.' {
                num.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        match num.parse::<f64>() {
            Ok(n) => Token::Number(n),
            Err(_) => {
                println!("Lexer error: invalid number '{}'", num);
                Token::Number(0.0)
            }
        }
    }

    fn lex_string(&mut self, delimiter: char) -> Token {
    let mut result = String::new();

    while let Some(c) = self.peek_char() {
        match c {
            ch if ch == delimiter => {
                self.next_char(); // consume closing quote
                break;
            }
            '\\' => {
                self.next_char(); // skip '\'
                if let Some(escaped) = self.next_char() {
                    result.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        '"' => '"',
                        '\'' => '\'',
                        '\\' => '\\',
                        other => other,
                    });
                }
            }
            _ => {
                result.push(c);
                self.next_char();
            }
        }
    }

    Token::String(result)
}


    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek_char() {
            self.next_char();
            if c == '\n' {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.pos).cloned()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char();
        self.pos += 1;
        ch
    }
}
