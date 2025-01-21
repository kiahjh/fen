#![allow(dead_code)]

use ast::{EnumDefinition, Field, FileNode, IOType, Primitive, StructDefinition, Type, Variant};
use lexer::Lexer;
use tokens::{Token, TokenKind};

pub mod ast;
pub mod codegen;
mod lexer;
mod tokens;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Expected {
        expected: &'static str,
        got: TokenKind,
    },
    WrongToken {
        expected: TokenKind,
        got: TokenKind,
    },
    UnexpectedEOF,
    Message(String),
    FromLexer(lexer::Error),
}

impl From<lexer::Error> for Error {
    fn from(lex_err: lexer::Error) -> Self {
        Self::FromLexer(lex_err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Expected { expected, got } => {
                write!(f, "Expected {expected}, got {got:?}")
            }
            Self::WrongToken { expected, got } => {
                write!(f, "Expected {expected:?}, got {got:?}")
            }
            Self::UnexpectedEOF => write!(f, "Unexpected EOF"),
            Self::Message(msg) => write!(f, "{msg}"),
            Self::FromLexer(lex_err) => write!(f, "Lexer error: {lex_err}"),
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    ast: FileNode,
    defined_types: Vec<String>,
}

impl Parser {
    #[must_use]
    pub fn new(file: &str) -> Self {
        Self {
            lexer: Lexer::new(file),
            ast: FileNode {
                name: String::new(),
                description: None,
                authed: false,
                input: None,
                output: None,
                structs: vec![],
                enums: vec![],
            },
            defined_types: vec![],
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if input is not valid.
    pub fn parse(&mut self) -> Result<FileNode, Error> {
        // skip to helper types so those are registered first and can be referenced in io types
        let has_helper_types = self.skip_to_helper_types()?;

        // if there is a helper type section (aka we found two rules), parse it
        if has_helper_types {
            self.parse_helper_types()?;
        }

        // reset the lexer and start parsing from the beginning
        self.lexer.reset();

        self.parse_metadata()?;
        self.parse_io()?;

        Ok(self.ast.clone())
    }

    fn skip_to_helper_types(&mut self) -> Result<bool, Error> {
        let mut rules_found = 0;
        while rules_found < 2 {
            let next_token = self.lexer.next_tok()?;
            match next_token {
                Some(Token {
                    kind: TokenKind::Rule,
                    index: _,
                }) => rules_found += 1,
                None => break,
                _ => continue,
            }
        }

        Ok(rules_found == 2)
    }

    fn parse_helper_types(&mut self) -> Result<(), Error> {
        // first look through and register all the types
        while let Some(tok) = self.lexer.next_tok()? {
            match tok.kind {
                TokenKind::At => {
                    self.expect_identifier()?;
                    continue;
                }
                TokenKind::Identifier(name) => self.defined_types.push(name),
                _ => {
                    return Err(Error::Expected {
                        expected: "an identifier",
                        got: tok.kind,
                    })
                }
            }
            let next_token = self.lexer.next_tok()?.ok_or(Error::UnexpectedEOF)?;
            if next_token.kind == TokenKind::LeftBrace {
                while let Some(tok) = self.lexer.next_tok()? {
                    if tok.kind == TokenKind::RightBrace {
                        break;
                    }
                }
            } else if next_token.kind == TokenKind::LeftParen {
                let mut open_inner_sets = 0;
                while let Some(tok) = self.lexer.next_tok()? {
                    if tok.kind == TokenKind::LeftParen {
                        open_inner_sets += 1;
                    }
                    if tok.kind == TokenKind::RightParen {
                        if open_inner_sets == 0 {
                            break;
                        }
                        open_inner_sets -= 1;
                    }
                }
            } else {
                return Err(Error::Expected {
                    expected: "a struct or enum definition >>>>>",
                    got: next_token.kind,
                });
            }
        }

        // then go back and parse them
        self.lexer.reset();
        self.skip_to_helper_types()?;
        let annotations = &mut vec![];
        while let Some(tok) = self.lexer.next_tok()? {
            match tok.kind {
                TokenKind::At => {
                    let annotation = self.expect_identifier()?;
                    annotations.push(annotation);
                }
                TokenKind::Identifier(name) => {
                    let next_token = self.lexer.peek_tok()?.ok_or(Error::UnexpectedEOF)?;
                    if next_token.kind == TokenKind::LeftBrace {
                        let struct_def =
                            self.parse_struct_definition(&name, annotations.clone())?;
                        annotations.clear();
                        self.ast.structs.push(struct_def);
                    } else if next_token.kind == TokenKind::LeftParen {
                        let enum_def = self.parse_enum_definition(&name, annotations.clone())?;
                        annotations.clear();
                        self.ast.enums.push(enum_def);
                    } else {
                        return Err(Error::Expected {
                            expected: "a struct or enum definition",
                            got: next_token.kind.clone(),
                        });
                    }
                }
                other => {
                    return Err(Error::Expected {
                        expected: "an identifier",
                        got: other,
                    })
                }
            }
        }

        Ok(())
    }

    fn parse_io(&mut self) -> Result<(), Error> {
        let at_token = self.expect_token(&TokenKind::At);
        if at_token.is_err() {
            return Err(Error::Message(
                "Route must have input, output, or both".to_string(),
            ));
        }

        let first_ident = self.expect_identifier()?;
        match first_ident.as_str() {
            "input" => {
                self.ast.input = Some(self.parse_io_type("input")?);
                let next_token = self.lexer.peek_tok()?;
                if next_token.is_none() {
                    return Ok(());
                }
                let next_token = next_token.unwrap();
                if next_token.kind == TokenKind::At {
                    self.expect_token(&TokenKind::At)?;
                    let ident = self.expect_identifier()?;
                    if ident == "output" {
                        self.ast.output = Some(self.parse_io_type("output")?);
                    } else {
                        return Err(Error::Expected {
                            expected: "output",
                            got: TokenKind::Identifier(ident),
                        });
                    }
                }
            }
            "output" => {
                self.ast.output = Some(self.parse_io_type("output")?);
            }
            _ => {
                return Err(Error::Expected {
                    expected: "input or output",
                    got: TokenKind::Identifier(first_ident),
                })
            }
        }

        Ok(())
    }

    fn parse_metadata(&mut self) -> Result<(), Error> {
        self.expect_token(&TokenKind::Identifier("name".to_string()))?;
        self.expect_token(&TokenKind::Colon)?;
        self.ast.name = self.expect_string_literal()?;

        while let Some(tok) = self.lexer.next_tok()? {
            match tok {
                Token {
                    kind: TokenKind::Identifier(name),
                    index: _,
                } => {
                    if name == "description" {
                        self.expect_token(&TokenKind::Colon)?;
                        self.ast.description = Some(self.expect_string_literal()?);
                    } else if name == "authed" {
                        self.expect_token(&TokenKind::Colon)?;
                        if let Some(Token {
                            kind: TokenKind::BoolLiteral(value),
                            index: _,
                        }) = self.lexer.next_tok()?
                        {
                            self.ast.authed = value;
                        } else {
                            return Err(Error::Expected {
                                expected: "a boolean literal",
                                got: TokenKind::Eof,
                            });
                        }
                    }
                }
                Token {
                    kind: TokenKind::Rule,
                    index: _,
                } => break,
                Token { kind, index: _ } => {
                    return Err(Error::Expected {
                        expected: "an identifier or rule",
                        got: kind,
                    });
                }
            }
        }

        Ok(())
    }

    fn parse_io_type(&mut self, name: &str) -> Result<IOType, Error> {
        let next_tok = self.lexer.peek_tok()?.ok_or(Error::UnexpectedEOF)?;
        match next_tok.kind {
            TokenKind::LeftBrace => Ok(IOType::Struct(self.parse_struct_definition(name, vec![])?)),
            TokenKind::LeftParen => Ok(IOType::Enum(self.parse_enum_definition(name, vec![])?)),
            TokenKind::LeftBracket
            | TokenKind::Identifier(_)
            | TokenKind::Int
            | TokenKind::Float
            | TokenKind::String
            | TokenKind::Bool
            | TokenKind::Date
            | TokenKind::Uuid => Ok(IOType::Type(self.parse_type()?)),
            _ => Err(Error::Expected {
                expected: "an inline struct, an inline enum, or a type",
                got: next_tok.kind.clone(),
            }),
        }
    }

    fn parse_struct_definition(
        &mut self,
        name: &str,
        annotations: Vec<String>,
    ) -> Result<StructDefinition, Error> {
        self.expect_token(&TokenKind::LeftBrace)?;

        let mut struct_def = StructDefinition {
            name: name.to_string(),
            fields: vec![],
            annotations,
        };

        while let Some(tok) = self.lexer.peek_tok()? {
            match tok.kind {
                TokenKind::RightBrace => break,
                TokenKind::Identifier(_) => struct_def.fields.push(self.parse_struct_field()?),
                _ => {
                    return Err(Error::Expected {
                        expected: "an identifier",
                        got: tok.kind.clone(),
                    });
                }
            }
        }
        self.expect_token(&TokenKind::RightBrace)?;

        Ok(struct_def)
    }

    fn parse_struct_field(&mut self) -> Result<Field, Error> {
        let name = self.expect_identifier()?;
        self.expect_token(&TokenKind::Colon)?;
        let t = self.parse_type()?;

        Ok(Field { name, t })
    }

    fn parse_enum_definition(
        &mut self,
        name: &str,
        annotations: Vec<String>,
    ) -> Result<EnumDefinition, Error> {
        self.expect_token(&TokenKind::LeftParen)?;

        let mut enum_def = EnumDefinition {
            name: name.to_string(),
            variants: vec![],
            annotations,
        };

        while let Some(tok) = self.lexer.peek_tok()? {
            match tok.kind {
                TokenKind::RightParen => break,
                TokenKind::Identifier(_) => enum_def.variants.push(self.parse_enum_variant()?),
                _ => {
                    return Err(Error::Expected {
                        expected: "an identifier",
                        got: tok.kind.clone(),
                    });
                }
            }
        }
        self.expect_token(&TokenKind::RightParen)?;

        Ok(enum_def)
    }

    fn parse_enum_variant(&mut self) -> Result<Variant, Error> {
        let name = self.expect_identifier()?;
        let next_token = self.lexer.peek_tok()?.ok_or(Error::UnexpectedEOF)?;
        let t = if next_token.kind == TokenKind::LeftParen {
            self.expect_token(&TokenKind::LeftParen)?;
            let t = self.parse_type()?;
            self.expect_token(&TokenKind::RightParen)?;
            Some(t)
        } else {
            None
        };

        Ok(Variant { name, t })
    }

    fn parse_type(&mut self) -> Result<Type, Error> {
        let first_token = self.lexer.peek_tok()?.ok_or(Error::Expected {
            expected: "a type",
            got: TokenKind::Eof,
        })?;

        let inner = match &first_token.kind {
            TokenKind::Identifier(name) => {
                if self.defined_types.contains(name) {
                    let name = name.to_string();
                    self.expect_identifier()?;
                    Type::Named(name)
                } else {
                    return Err(Error::Message(format!(
                        "Reference to undefined type: {name}"
                    )));
                }
            }
            TokenKind::Int => {
                self.expect_token(&TokenKind::Int)?;
                Type::Primitive(Primitive::Int)
            }
            TokenKind::Float => {
                self.expect_token(&TokenKind::Float)?;
                Type::Primitive(Primitive::Float)
            }
            TokenKind::Date => {
                self.expect_token(&TokenKind::Date)?;
                Type::Primitive(Primitive::Date)
            }
            TokenKind::Uuid => {
                self.expect_token(&TokenKind::Uuid)?;
                Type::Primitive(Primitive::Uuid)
            }
            TokenKind::String => {
                self.expect_token(&TokenKind::String)?;
                Type::Primitive(Primitive::String)
            }
            TokenKind::Bool => {
                self.expect_token(&TokenKind::Bool)?;
                Type::Primitive(Primitive::Bool)
            }
            TokenKind::LeftBracket => self.parse_array()?,
            _ => {
                return Err(Error::Expected {
                    expected: "a type",
                    got: first_token.kind.clone(),
                })
            }
        };

        // check to see if it's optional (by looking for '?')
        let next_token = self.lexer.peek_tok()?;
        let mut is_optional: bool = false;

        if let Some(token) = next_token {
            is_optional = token.kind == TokenKind::QuestionMark;
        }

        if is_optional {
            self.expect_token(&TokenKind::QuestionMark)?;
            Ok(Type::Optional(Box::new(inner)))
        } else {
            Ok(inner)
        }
    }

    fn parse_array(&mut self) -> Result<Type, Error> {
        self.expect_token(&TokenKind::LeftBracket)?;
        let arr = Type::Array(Box::new(self.parse_type()?));
        self.expect_token(&TokenKind::RightBracket)?;

        Ok(arr)
    }

    fn expect_token(&mut self, kind: &TokenKind) -> Result<(), Error> {
        match self.lexer.next_tok()? {
            Some(token) if token.kind == *kind => Ok(()),
            Some(tok) => Err(Error::WrongToken {
                expected: kind.clone(),
                got: tok.kind,
            }),
            None => Err(Error::WrongToken {
                expected: kind.clone(),
                got: TokenKind::Eof,
            }),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, Error> {
        match self.lexer.next_tok()? {
            Some(Token {
                kind: TokenKind::Identifier(name),
                index: _,
            }) => Ok(name),
            Some(tok) => Err(Error::Expected {
                expected: "an identifier",
                got: tok.kind,
            }),
            None => Err(Error::Expected {
                expected: "an identifier",
                got: TokenKind::Eof,
            }),
        }
    }

    fn expect_string_literal(&mut self) -> Result<String, Error> {
        match self.lexer.next_tok()? {
            Some(Token {
                kind: TokenKind::StringLiteral(value),
                index: _,
            }) => Ok(value),
            Some(tok) => Err(Error::Expected {
                expected: "a string literal",
                got: tok.kind,
            }),
            None => Err(Error::Expected {
                expected: "a string literal",
                got: TokenKind::Eof,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // TODO: figure this out, ideally should take a &FileNode
    #[allow(clippy::needless_pass_by_value)]
    fn expect_ast(file: &str, expected: FileNode) {
        let mut parser = Parser::new(file);

        assert_eq!(parser.parse().unwrap(), expected);
    }

    fn expect_error(file: &str, expected: &Error) {
        let mut parser = Parser::new(file);

        let err = parser.parse().unwrap_err();
        assert_eq!(err, *expected);
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn parses_metadata_and_io() {
        // basic types:
        expect_ast(
            r#"
            name: "Test"
            description: "This is a test"
            authed: true

            ---

            @input Int

            @output String
            "#,
            FileNode {
                name: "Test".to_string(),
                description: Some("This is a test".to_string()),
                authed: true,
                input: Some(IOType::Type(Type::Primitive(ast::Primitive::Int))),
                output: Some(IOType::Type(Type::Primitive(ast::Primitive::String))),
                structs: vec![],
                enums: vec![],
            },
        );

        // with no description, no auth, and no input
        expect_ast(
            r#"
            name: "Test"

            ---

            @output [String]?
            "#,
            FileNode {
                name: "Test".to_string(),
                description: None,
                authed: false,
                input: None,
                output: Some(IOType::Type(Type::Optional(Box::new(Type::Array(
                    Box::new(Type::Primitive(Primitive::String)),
                ))))),
                structs: vec![],
                enums: vec![],
            },
        );

        // no input or output is an error:
        expect_error(
            r#"
            name: "Test"
            description: "This is a test"
            authed: true

            ---
            "#,
            &Error::Message("Route must have input, output, or both".to_string()),
        );

        // just input no output
        expect_ast(
            r#"
            name: "CompleteTodo"

            ---

            @input {
              id: UUID
            }
            "#,
            FileNode {
                name: "CompleteTodo".to_string(),
                description: None,
                authed: false,
                input: Some(IOType::Struct(StructDefinition {
                    name: "input".to_string(),
                    fields: vec![Field {
                        name: "id".to_string(),
                        t: Type::Primitive(Primitive::Uuid),
                    }],
                    annotations: vec![],
                })),
                output: None,
                structs: vec![],
                enums: vec![],
            },
        );

        // inline types
        expect_ast(
            r#"
            name: "Test"

            ---

            @input {
              username: String
              password: String
            }

            @output (
              foo
              bar(Int)
            )
            "#,
            FileNode {
                name: "Test".to_string(),
                description: None,
                authed: false,
                input: Some(IOType::Struct(StructDefinition {
                    name: "input".to_string(),
                    fields: vec![
                        Field {
                            name: "username".to_string(),
                            t: Type::Primitive(Primitive::String),
                        },
                        Field {
                            name: "password".to_string(),
                            t: Type::Primitive(Primitive::String),
                        },
                    ],
                    annotations: vec![],
                })),
                output: Some(IOType::Enum(EnumDefinition {
                    name: "output".to_string(),
                    variants: vec![
                        Variant {
                            name: "foo".to_string(),
                            t: None,
                        },
                        Variant {
                            name: "bar".to_string(),
                            t: Some(Type::Primitive(Primitive::Int)),
                        },
                    ],
                    annotations: vec![],
                })),
                structs: vec![],
                enums: vec![],
            },
        );
    }

    #[test]
    fn helper_types() {
        expect_ast(
            r#"
            name: "Login"
            description: "Login to the system"
            authed: false

            ---

            @input {
              username: String
              password: String
            }

            @output Token

            ---

            Token {
              token: String
              expiry: Expiration
            }

            Expiration (
              standard(Date)
              never
            )
            "#,
            FileNode {
                name: "Login".to_string(),
                description: Some("Login to the system".to_string()),
                authed: false,
                input: Some(IOType::Struct(StructDefinition {
                    name: "input".to_string(),
                    fields: vec![
                        Field {
                            name: "username".to_string(),
                            t: Type::Primitive(Primitive::String),
                        },
                        Field {
                            name: "password".to_string(),
                            t: Type::Primitive(Primitive::String),
                        },
                    ],
                    annotations: vec![],
                })),
                output: Some(IOType::Type(Type::Named("Token".to_string()))),
                structs: vec![StructDefinition {
                    name: "Token".to_string(),
                    fields: vec![
                        Field {
                            name: "token".to_string(),
                            t: Type::Primitive(Primitive::String),
                        },
                        Field {
                            name: "expiry".to_string(),
                            t: Type::Named("Expiration".to_string()),
                        },
                    ],
                    annotations: vec![],
                }],
                enums: vec![EnumDefinition {
                    name: "Expiration".to_string(),
                    variants: vec![
                        Variant {
                            name: "standard".to_string(),
                            t: Some(Type::Primitive(Primitive::Date)),
                        },
                        Variant {
                            name: "never".to_string(),
                            t: None,
                        },
                    ],
                    annotations: vec![],
                }],
            },
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn complex_example() {
        expect_ast(
            r#"
            name: "GetPeopleInfo"
            description: "Get information about people"
            authed: true

            ---

            @input {
              ids: [UUID]
            }

            @output [PersonInfo]

            ---

            @someAnnotation
            PersonInfo {
              id: UUID
              born: Date
              spouse: PersonInfo?
              children: [PersonInfo]
              job: Work
            }

            Work {
              title: String
              hours: Int
              place: WorkPlace
            }

            @anotherAnnotation
            @andAnother
            WorkPlace (
              at_home
              on_site
              hybrid
            )
            "#,
            FileNode {
                name: "GetPeopleInfo".to_string(),
                description: Some("Get information about people".to_string()),
                authed: true,
                input: Some(IOType::Struct(StructDefinition {
                    name: "input".to_string(),
                    fields: vec![Field {
                        name: "ids".to_string(),
                        t: Type::Array(Box::new(Type::Primitive(Primitive::Uuid))),
                    }],
                    annotations: vec![],
                })),
                output: Some(IOType::Type(Type::Array(Box::new(Type::Named(
                    "PersonInfo".to_string(),
                ))))),
                structs: vec![
                    StructDefinition {
                        name: "PersonInfo".to_string(),
                        fields: vec![
                            Field {
                                name: "id".to_string(),
                                t: Type::Primitive(Primitive::Uuid),
                            },
                            Field {
                                name: "born".to_string(),
                                t: Type::Primitive(Primitive::Date),
                            },
                            Field {
                                name: "spouse".to_string(),
                                t: Type::Optional(Box::new(Type::Named("PersonInfo".to_string()))),
                            },
                            Field {
                                name: "children".to_string(),
                                t: Type::Array(Box::new(Type::Named("PersonInfo".to_string()))),
                            },
                            Field {
                                name: "job".to_string(),
                                t: Type::Named("Work".to_string()),
                            },
                        ],
                        annotations: vec!["someAnnotation".to_string()],
                    },
                    StructDefinition {
                        name: "Work".to_string(),
                        fields: vec![
                            Field {
                                name: "title".to_string(),
                                t: Type::Primitive(Primitive::String),
                            },
                            Field {
                                name: "hours".to_string(),
                                t: Type::Primitive(Primitive::Int),
                            },
                            Field {
                                name: "place".to_string(),
                                t: Type::Named("WorkPlace".to_string()),
                            },
                        ],
                        annotations: vec![],
                    },
                ],
                enums: vec![EnumDefinition {
                    name: "WorkPlace".to_string(),
                    variants: vec![
                        Variant {
                            name: "at_home".to_string(),
                            t: None,
                        },
                        Variant {
                            name: "on_site".to_string(),
                            t: None,
                        },
                        Variant {
                            name: "hybrid".to_string(),
                            t: None,
                        },
                    ],
                    annotations: vec!["anotherAnnotation".to_string(), "andAnother".to_string()],
                }],
            },
        );
    }
}
