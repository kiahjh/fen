use crate::tokens::{Token, TokenKind};

#[derive(PartialEq, Eq, Debug)]
pub struct Error {
    pub message: String,
    pub position: usize,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error at position {}: {}", self.position, self.message)
    }
}

impl Error {
    fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

pub struct Lexer {
    chars: Vec<u8>,
    pos: usize,
    has_errored: bool,
    peek_token: Option<Token>,
}

impl Lexer {
    pub fn new(file: impl Into<String>) -> Self {
        let file: String = file.into();
        Self {
            chars: file.as_bytes().to_vec(),
            pos: 0,
            has_errored: false,
            peek_token: None,
        }
    }

    /// # Errors
    ///
    /// Returns an error if the lexer encounters a forbidden character or syntax error.
    pub fn peek_tok(&mut self) -> Result<Option<&Token>, Error> {
        if self.peek_token.is_none() {
            self.peek_token = self.next_tok()?;
        }
        Ok(self.peek_token.as_ref())
    }

    /// # Errors
    ///
    /// Returns an error if the lexer encounters a forbidden character or syntax error.
    pub fn next_tok(&mut self) -> Result<Option<Token>, Error> {
        if let Some(peeked) = self.peek_token.take() {
            return Ok(Some(peeked));
        }

        // skip over whitespace
        while self.peek_char().map_or(false, u8::is_ascii_whitespace) {
            self.pos += 1;
        }

        let mut return_val: Result<Option<Token>, Error> = Ok(None);

        if self.has_errored {
            return return_val;
        }

        if let Some(c) = self.next_char() {
            match c {
                // single char tokens
                b'{' => return_val = Ok(Some(Token::new(TokenKind::LeftBrace, self.pos - 1))),
                b'}' => return_val = Ok(Some(Token::new(TokenKind::RightBrace, self.pos - 1))),
                b'(' => return_val = Ok(Some(Token::new(TokenKind::LeftParen, self.pos - 1))),
                b')' => return_val = Ok(Some(Token::new(TokenKind::RightParen, self.pos - 1))),
                b'[' => return_val = Ok(Some(Token::new(TokenKind::LeftBracket, self.pos - 1))),
                b']' => return_val = Ok(Some(Token::new(TokenKind::RightBracket, self.pos - 1))),
                b',' => return_val = Ok(Some(Token::new(TokenKind::Comma, self.pos - 1))),
                b':' => return_val = Ok(Some(Token::new(TokenKind::Colon, self.pos - 1))),
                b'?' => return_val = Ok(Some(Token::new(TokenKind::QuestionMark, self.pos - 1))),

                // illegal chars
                b'%' => return_val = Err(Error::new("Forbidden character '%'", self.pos - 1)),
                b'@' => return_val = Err(Error::new("Forbidden character '@'", self.pos - 1)),
                b'!' => return_val = Err(Error::new("Forbidden character '!'", self.pos - 1)),
                b'&' => return_val = Err(Error::new("Forbidden character '&'", self.pos - 1)),
                b'*' => return_val = Err(Error::new("Forbidden character '*'", self.pos - 1)),
                b'+' => return_val = Err(Error::new("Forbidden character '+'", self.pos - 1)),
                b'-' => return_val = Err(Error::new("Forbidden character '-'", self.pos - 1)),
                b'/' => return_val = Err(Error::new("Forbidden character '/'", self.pos - 1)),
                b'<' => return_val = Err(Error::new("Forbidden character '<'", self.pos - 1)),
                b'>' => return_val = Err(Error::new("Forbidden character '>'", self.pos - 1)),
                b'=' => return_val = Err(Error::new("Forbidden character '='", self.pos - 1)),
                b'.' => return_val = Err(Error::new("Forbidden character '.'", self.pos - 1)),
                b';' => return_val = Err(Error::new("Forbidden character ';'", self.pos - 1)),
                b'"' => return_val = Err(Error::new("Forbidden character '\"'", self.pos - 1)),
                b'\'' => return_val = Err(Error::new("Forbidden character '''", self.pos - 1)),
                b'\\' => return_val = Err(Error::new("Forbidden character '\\'", self.pos - 1)),
                b'`' => return_val = Err(Error::new("Forbidden character '`'", self.pos - 1)),
                b'~' => return_val = Err(Error::new("Forbidden character '~'", self.pos - 1)),
                b'|' => return_val = Err(Error::new("Forbidden character '|'", self.pos - 1)),
                b'^' => return_val = Err(Error::new("Forbidden character '^'", self.pos - 1)),

                // multichar tokens
                _ => {
                    return_val = Ok(Some(Token {
                        kind: self.parse_multichar_token()?,
                        index: 0,
                    }));
                }
            }
        }

        if return_val.is_err() {
            self.has_errored = true;
        }

        return_val
    }

    fn next_char(&mut self) -> Option<&u8> {
        let c = self.chars.get(self.pos);
        match c {
            Some(c) => {
                self.pos += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn peek_char(&self) -> Option<&u8> {
        self.chars.get(self.pos)
    }

    fn parse_multichar_token(&mut self) -> Result<TokenKind, Error> {
        let initial_pos = self.pos - 1;

        while self.peek_char().map_or(false, |c| match c {
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b',' | b':' | b'?' | b'%' | b'@' | b'!'
            | b'&' | b'*' | b'+' | b'-' | b'/' | b'<' | b'>' | b'=' | b'.' | b';' | b'"'
            | b'\'' | b'\\' | b'`' | b'~' | b'|' | b'^' => false,
            _ => !c.is_ascii_whitespace(),
        }) {
            self.pos += 1;
        }

        let slice = &self.chars[initial_pos..self.pos];

        match slice {
            b"struct" => Ok(TokenKind::Struct),
            b"enum" => Ok(TokenKind::Enum),
            b"Int" => Ok(TokenKind::Int),
            b"Float" => Ok(TokenKind::Float),
            b"Date" => Ok(TokenKind::Date),
            b"UUID" => Ok(TokenKind::Uuid),
            b"String" => Ok(TokenKind::String),
            b"Bool" => Ok(TokenKind::Bool),
            _ => std::str::from_utf8(slice).map_or(
                Err(Error {
                    message: "Invalid UTF8 encoding".to_string(),
                    position: initial_pos,
                }),
                |s| Ok(TokenKind::Identifier(s.to_string())),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_tokens(file: &str, expected: &[TokenKind]) {
        let mut lexer = Lexer::new(file);
        let mut tokens = vec![];
        while let Ok(Some(token)) = lexer.next_tok() {
            tokens.push(token.kind);
        }
        assert_eq!(tokens, expected);
        assert_eq!(lexer.next_tok(), Ok(None));
    }

    fn expect_error(file: &str, expected: Error) {
        let mut lexer = Lexer::new(file);
        assert_eq!(lexer.next_tok(), Err(expected));
    }

    #[test]
    fn single_char_tokens() {
        expect_tokens("{", &[TokenKind::LeftBrace]);
        expect_tokens("}", &[TokenKind::RightBrace]);
        expect_tokens("(", &[TokenKind::LeftParen]);
        expect_tokens(")", &[TokenKind::RightParen]);
        expect_tokens("[", &[TokenKind::LeftBracket]);
        expect_tokens("]", &[TokenKind::RightBracket]);
        expect_tokens(",", &[TokenKind::Comma]);
        expect_tokens(":", &[TokenKind::Colon]);
        expect_tokens("?", &[TokenKind::QuestionMark]);

        expect_tokens("{ }", &[TokenKind::LeftBrace, TokenKind::RightBrace]);

        expect_error("%", Error::new("Forbidden character '%'", 0));
    }

    #[test]
    fn keywords() {
        expect_tokens("struct", &[TokenKind::Struct]);
        expect_tokens("enum", &[TokenKind::Enum]);
    }

    #[test]
    fn identifier() {
        expect_tokens("foo", &[TokenKind::Identifier("foo".to_string())]);
        expect_tokens(
            "fLIadi89av$FEljk__faekj",
            &[TokenKind::Identifier("fLIadi89av$FEljk__faekj".to_string())],
        );
    }

    #[test]
    fn mid_ident_bad_char() {
        let mut lexer = Lexer::new("foo%bar {}");

        // should initially return the identifier "foo"
        assert_eq!(
            lexer.next_tok(),
            Ok(Some(Token::new(
                TokenKind::Identifier("foo".to_string()),
                0
            )))
        );

        // then throw an error when it hits the '%' character
        assert_eq!(
            lexer.next_tok(),
            Err(Error::new("Forbidden character '%'", 3))
        );

        // after the lexer has errored, it should return None forever
        assert_eq!(lexer.next_tok(), Ok(None));
        assert_eq!(lexer.next_tok(), Ok(None));
    }

    #[test]
    fn all_syntax() {
        expect_tokens(
            r"
                enum Something {
                  one,
                  two(Int),
                  three([[Bool]?]?),
                }

                struct Test {
                  foo: Int,
                  bar: String,
                  list: [Float],
                  algo: Date?,
                  algo_mas: [UUID],
                  several_somethings: [Something],
                }
            ",
            &[
                TokenKind::Enum,
                TokenKind::Identifier("Something".to_string()),
                TokenKind::LeftBrace,
                TokenKind::Identifier("one".to_string()),
                TokenKind::Comma,
                TokenKind::Identifier("two".to_string()),
                TokenKind::LeftParen,
                TokenKind::Int,
                TokenKind::RightParen,
                TokenKind::Comma,
                TokenKind::Identifier("three".to_string()),
                TokenKind::LeftParen,
                TokenKind::LeftBracket,
                TokenKind::LeftBracket,
                TokenKind::Bool,
                TokenKind::RightBracket,
                TokenKind::QuestionMark,
                TokenKind::RightBracket,
                TokenKind::QuestionMark,
                TokenKind::RightParen,
                TokenKind::Comma,
                TokenKind::RightBrace,
                TokenKind::Struct,
                TokenKind::Identifier("Test".to_string()),
                TokenKind::LeftBrace,
                TokenKind::Identifier("foo".to_string()),
                TokenKind::Colon,
                TokenKind::Int,
                TokenKind::Comma,
                TokenKind::Identifier("bar".to_string()),
                TokenKind::Colon,
                TokenKind::String,
                TokenKind::Comma,
                TokenKind::Identifier("list".to_string()),
                TokenKind::Colon,
                TokenKind::LeftBracket,
                TokenKind::Float,
                TokenKind::RightBracket,
                TokenKind::Comma,
                TokenKind::Identifier("algo".to_string()),
                TokenKind::Colon,
                TokenKind::Date,
                TokenKind::QuestionMark,
                TokenKind::Comma,
                TokenKind::Identifier("algo_mas".to_string()),
                TokenKind::Colon,
                TokenKind::LeftBracket,
                TokenKind::Uuid,
                TokenKind::RightBracket,
                TokenKind::Comma,
                TokenKind::Identifier("several_somethings".to_string()),
                TokenKind::Colon,
                TokenKind::LeftBracket,
                TokenKind::Identifier("Something".to_string()),
                TokenKind::RightBracket,
                TokenKind::Comma,
                TokenKind::RightBrace,
            ],
        );
    }
}
