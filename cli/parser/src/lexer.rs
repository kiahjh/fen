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

    pub fn reset(&mut self) {
        self.pos = 0;
        self.has_errored = false;
        self.peek_token = None;
    }

    pub(crate) fn peek_tok(&mut self) -> Result<Option<&Token>, Error> {
        if self.peek_token.is_none() {
            self.peek_token = self.next_tok()?;
        }
        Ok(self.peek_token.as_ref())
    }

    pub(crate) fn next_tok(&mut self) -> Result<Option<Token>, Error> {
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
                b'@' => return_val = Ok(Some(Token::new(TokenKind::At, self.pos - 1))),
                b':' => return_val = Ok(Some(Token::new(TokenKind::Colon, self.pos - 1))),
                b'?' => return_val = Ok(Some(Token::new(TokenKind::QuestionMark, self.pos - 1))),

                // string literals
                b'"' => {
                    return_val = Ok(Some(Token::new(
                        TokenKind::StringLiteral(self.parse_string_literal()?),
                        self.pos - 1,
                    )));
                }

                // illegal chars
                b'%' => return_val = Err(Error::new("Forbidden character '%'", self.pos - 1)),
                b'!' => return_val = Err(Error::new("Forbidden character '!'", self.pos - 1)),
                b'&' => return_val = Err(Error::new("Forbidden character '&'", self.pos - 1)),
                b'*' => return_val = Err(Error::new("Forbidden character '*'", self.pos - 1)),
                b'+' => return_val = Err(Error::new("Forbidden character '+'", self.pos - 1)),
                b'/' => return_val = Err(Error::new("Forbidden character '/'", self.pos - 1)),
                b'<' => return_val = Err(Error::new("Forbidden character '<'", self.pos - 1)),
                b'>' => return_val = Err(Error::new("Forbidden character '>'", self.pos - 1)),
                b'=' => return_val = Err(Error::new("Forbidden character '='", self.pos - 1)),
                b'.' => return_val = Err(Error::new("Forbidden character '.'", self.pos - 1)),
                b';' => return_val = Err(Error::new("Forbidden character ';'", self.pos - 1)),
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

    fn parse_string_literal(&mut self) -> Result<String, Error> {
        let initial_pos = self.pos - 1;

        while self.peek_char().map_or(false, |c| *c != b'"') {
            self.pos += 1;
        }

        // consume the closing quote
        self.pos += 1;

        let slice = &self.chars[initial_pos + 1..self.pos - 1]; // exclude the quotes

        std::str::from_utf8(slice).map_or(
            Err(Error {
                message: "Invalid UTF8 encoding".to_string(),
                position: initial_pos,
            }),
            |s| Ok(s.to_string()),
        )
    }

    fn parse_multichar_token(&mut self) -> Result<TokenKind, Error> {
        let initial_pos = self.pos - 1;

        while self.peek_char().map_or(false, |c| match c {
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b',' | b':' | b'?' | b'%' | b'@' | b'!'
            | b'&' | b'*' | b'+' | b'/' | b'<' | b'>' | b'=' | b'.' | b';' | b'"' | b'\''
            | b'\\' | b'`' | b'~' | b'|' | b'^' => false,
            _ => !c.is_ascii_whitespace(),
        }) {
            self.pos += 1;
        }

        let slice = &self.chars[initial_pos..self.pos];

        match slice {
            b"Int" => Ok(TokenKind::Int),
            b"Float" => Ok(TokenKind::Float),
            b"Date" => Ok(TokenKind::Date),
            b"UUID" => Ok(TokenKind::Uuid),
            b"String" => Ok(TokenKind::String),
            b"Bool" => Ok(TokenKind::Bool),
            b"true" => Ok(TokenKind::BoolLiteral(true)),
            b"false" => Ok(TokenKind::BoolLiteral(false)),
            b"---" => Ok(TokenKind::Rule),
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
    use pretty_assertions::assert_eq;

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
        expect_tokens("[", &[TokenKind::LeftBracket]);
        expect_tokens("]", &[TokenKind::RightBracket]);
        expect_tokens("(", &[TokenKind::LeftParen]);
        expect_tokens(")", &[TokenKind::RightParen]);
        expect_tokens(":", &[TokenKind::Colon]);
        expect_tokens("?", &[TokenKind::QuestionMark]);
        expect_tokens("@", &[TokenKind::At]);

        expect_tokens("{ }", &[TokenKind::LeftBrace, TokenKind::RightBrace]);

        expect_error("%", Error::new("Forbidden character '%'", 0));
    }

    #[test]
    fn string_literals() {
        expect_tokens(
            r#""hello world""#,
            &[TokenKind::StringLiteral("hello world".to_string())],
        );
        expect_tokens(
            r#""hello world" "another string""#,
            &[
                TokenKind::StringLiteral("hello world".to_string()),
                TokenKind::StringLiteral("another string".to_string()),
            ],
        );
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
        let mut lexer = Lexer::new("Foo%bar {}");

        // should initially return the identifier "foo"
        assert_eq!(
            lexer.next_tok(),
            Ok(Some(Token::new(
                TokenKind::Identifier("Foo".to_string()),
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
            r#"
            name: "GetTodos"
            description: "Get all todos for a user"
            authed: true

            ---

            @input { user_id: String }

            @output [Todo]

            ---

            Todo {
              name: String
              due: Date?
              priority: Priority?
              subtasks: [Todo]
            }

            Priority (
              low
              medium
              high
              other(String)
            )
            "#,
            &[
                TokenKind::Identifier("name".to_string()),
                TokenKind::Colon,
                TokenKind::StringLiteral("GetTodos".to_string()),
                TokenKind::Identifier("description".to_string()),
                TokenKind::Colon,
                TokenKind::StringLiteral("Get all todos for a user".to_string()),
                TokenKind::Identifier("authed".to_string()),
                TokenKind::Colon,
                TokenKind::BoolLiteral(true),
                TokenKind::Rule,
                TokenKind::At,
                TokenKind::Identifier("input".to_string()),
                TokenKind::LeftBrace,
                TokenKind::Identifier("user_id".to_string()),
                TokenKind::Colon,
                TokenKind::String,
                TokenKind::RightBrace,
                TokenKind::At,
                TokenKind::Identifier("output".to_string()),
                TokenKind::LeftBracket,
                TokenKind::Identifier("Todo".to_string()),
                TokenKind::RightBracket,
                TokenKind::Rule,
                TokenKind::Identifier("Todo".to_string()),
                TokenKind::LeftBrace,
                TokenKind::Identifier("name".to_string()),
                TokenKind::Colon,
                TokenKind::String,
                TokenKind::Identifier("due".to_string()),
                TokenKind::Colon,
                TokenKind::Date,
                TokenKind::QuestionMark,
                TokenKind::Identifier("priority".to_string()),
                TokenKind::Colon,
                TokenKind::Identifier("Priority".to_string()),
                TokenKind::QuestionMark,
                TokenKind::Identifier("subtasks".to_string()),
                TokenKind::Colon,
                TokenKind::LeftBracket,
                TokenKind::Identifier("Todo".to_string()),
                TokenKind::RightBracket,
                TokenKind::RightBrace,
                TokenKind::Identifier("Priority".to_string()),
                TokenKind::LeftParen,
                TokenKind::Identifier("low".to_string()),
                TokenKind::Identifier("medium".to_string()),
                TokenKind::Identifier("high".to_string()),
                TokenKind::Identifier("other".to_string()),
                TokenKind::LeftParen,
                TokenKind::String,
                TokenKind::RightParen,
                TokenKind::RightParen,
            ],
        );
    }
}
