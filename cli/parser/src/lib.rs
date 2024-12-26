use crate::{
    ast::{EnumDefinition, Field, FileNode, Primitive, StructDefinition, Type, Variant},
    lexer::Lexer,
    tokens::{Token, TokenKind},
};

pub mod ast;
pub mod lexer;
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
    cur_entity: Option<String>,
}

impl Parser {
    #[must_use]
    pub const fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            ast: FileNode {
                structs: vec![],
                enums: vec![],
            },
            cur_entity: None,
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if input is not valid.
    pub fn parse(&mut self) -> Result<&FileNode, Error> {
        while let Some(tok) = self.lexer.next_tok()? {
            match tok {
                Token {
                    kind: TokenKind::Struct,
                    index: _,
                } => {
                    let parsed_struct = self.parse_struct_definition()?;
                    self.ast.structs.push(parsed_struct);
                }
                Token {
                    kind: TokenKind::Enum,
                    index: _,
                } => {
                    let parsed_enum = self.parse_enum_definition()?;
                    self.ast.enums.push(parsed_enum);
                }
                _ => {
                    return Err(Error::Message(format!(
                        "Only top-level structs and enums are supported, got {:?}",
                        tok.kind
                    )))
                }
            }
        }

        Ok(&self.ast)
    }

    fn parse_struct_definition(&mut self) -> Result<StructDefinition, Error> {
        let name = self.expect_identifier()?;
        self.cur_entity = Some(name.clone());
        self.expect_token(&TokenKind::LeftBrace)?;

        let mut struct_def = StructDefinition {
            name,
            fields: vec![],
        };

        while let Some(tok) = self.lexer.peek_tok()? {
            match tok.kind {
                TokenKind::RightBrace => break,
                TokenKind::Identifier(_) => {
                    struct_def.fields.push(self.parse_struct_field()?);
                    let next_token = self.lexer.peek_tok()?.ok_or(Error::UnexpectedEOF)?;
                    if next_token.kind == TokenKind::Comma {
                        self.expect_token(&TokenKind::Comma)?;
                    } else if next_token.kind != TokenKind::RightBrace {
                        return Err(Error::WrongToken {
                            expected: TokenKind::Comma,
                            got: next_token.kind.clone(),
                        });
                    }
                }
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

    fn parse_enum_definition(&mut self) -> Result<EnumDefinition, Error> {
        let name = self.expect_identifier()?;
        self.cur_entity = Some(name.clone());
        self.expect_token(&TokenKind::LeftBrace)?;

        let mut enum_def = EnumDefinition {
            name,
            variants: vec![],
        };

        while let Some(tok) = self.lexer.peek_tok()? {
            match tok.kind {
                TokenKind::RightBrace => break,
                TokenKind::Identifier(_) => {
                    enum_def.variants.push(self.parse_enum_variant()?);
                    let next_token = self.lexer.peek_tok()?.ok_or(Error::UnexpectedEOF)?;
                    if next_token.kind == TokenKind::Comma {
                        self.expect_token(&TokenKind::Comma)?;
                    } else if next_token.kind != TokenKind::RightBrace {
                        return Err(Error::WrongToken {
                            expected: TokenKind::Comma,
                            got: next_token.kind.clone(),
                        });
                    }
                }
                _ => {
                    return Err(Error::Expected {
                        expected: "an identifier",
                        got: tok.kind.clone(),
                    });
                }
            }
        }
        self.expect_token(&TokenKind::RightBrace)?;

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
                if self.ast.structs.iter().any(|s| s.name == *name)
                    || self.ast.enums.iter().any(|e| e.name == *name)
                    || Some(name) == self.cur_entity.as_ref()
                {
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
        let next_token = self.lexer.peek_tok()?.ok_or(Error::UnexpectedEOF)?;
        let is_optional = next_token.kind == TokenKind::QuestionMark;

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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::{EnumDefinition, Primitive, Variant};

    fn expect_ast(file: &str, expected: &FileNode) {
        let lexer = Lexer::new(file);
        let mut parser = Parser::new(lexer);

        assert_eq!(parser.parse().unwrap(), expected);
    }

    fn expect_error(file: &str, expected: &Error) {
        let lexer = Lexer::new(file);
        let mut parser = Parser::new(lexer);

        let err = parser.parse().unwrap_err();
        assert_eq!(err, *expected);
    }

    #[test]
    fn parses_empty_struct() {
        expect_ast(
            "struct Foo {}",
            &FileNode {
                enums: vec![],
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![],
                }],
            },
        );
    }

    #[test]
    fn parses_struct_with_fields() {
        expect_ast(
            "struct Foo { bar: Int }",
            &FileNode {
                enums: vec![],
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![Field {
                        name: "bar".to_string(),
                        t: Type::Primitive(Primitive::Int),
                    }],
                }],
            },
        );

        expect_ast(
            "struct Foo { bar: Int, baz: String }",
            &FileNode {
                enums: vec![],
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![
                        Field {
                            name: "bar".to_string(),
                            t: Type::Primitive(Primitive::Int),
                        },
                        Field {
                            name: "baz".to_string(),
                            t: Type::Primitive(Primitive::String),
                        },
                    ],
                }],
            },
        );

        // commas are necessary between fields
        expect_error(
            "struct Foo { bar: Int baz: String }",
            &Error::WrongToken {
                expected: TokenKind::Comma,
                got: TokenKind::Identifier("baz".to_string()),
            },
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn parses_structs_with_complex_types() {
        // optional
        expect_ast(
            "struct Foo { bar: Int? }",
            &FileNode {
                enums: vec![],
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![Field {
                        name: "bar".to_string(),
                        t: Type::Optional(Box::new(Type::Primitive(Primitive::Int))),
                    }],
                }],
            },
        );

        // array
        expect_ast(
            "struct Foo { bar: [Int] }",
            &FileNode {
                enums: vec![],
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![Field {
                        name: "bar".to_string(),
                        t: Type::Array(Box::new(Type::Primitive(Primitive::Int))),
                    }],
                }],
            },
        );

        // array of optional
        expect_ast(
            "struct Foo { bar: [Int?] }",
            &FileNode {
                enums: vec![],
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![Field {
                        name: "bar".to_string(),
                        t: Type::Array(Box::new(Type::Optional(Box::new(Type::Primitive(
                            Primitive::Int,
                        ))))),
                    }],
                }],
            },
        );

        // optional array
        expect_ast(
            "struct Foo { bar: [Int]? }",
            &FileNode {
                enums: vec![],
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![Field {
                        name: "bar".to_string(),
                        t: Type::Optional(Box::new(Type::Array(Box::new(Type::Primitive(
                            Primitive::Int,
                        ))))),
                    }],
                }],
            },
        );

        // complicated mess
        expect_ast(
            "struct Person { name: String, hat_brand: String?, children: [Person], something_else: [[[Int?]?]] }",
            &FileNode {
                structs: vec![StructDefinition {
                    name: "Person".to_string(),
                    fields: vec![
                        Field {
                            name: "name".to_string(),
                            t: Type::Primitive(Primitive::String),
                        },
                        Field {
                            name: "hat_brand".to_string(),
                            t: Type::Optional(
                                Box::new(Type::Primitive(Primitive::String)),
                            ),
                        },
                        Field {
                            name: "children".to_string(),
                            t: Type::Array(
                                Box::new(Type::Named("Person".to_string())),
                            ),
                        },
                        Field {
                            name: "something_else".to_string(),
                            t: Type::Array(
                                Box::new(Type::Array(
                                     Box::new(Type::Optional(
                                        Box::new(Type::Array(
                                            Box::new(Type::Optional(
                                                Box::new(Type::Primitive(Primitive::Int)),
                                            )),
                                        )),
                                    )),
                                )),
                            ),
                        }
                    ],
                }],
                enums: vec![],
            },
        );
    }

    #[test]
    fn parses_multiple_structs() {
        expect_ast(
            "struct Foo {} struct Bar {}",
            &FileNode {
                structs: vec![
                    StructDefinition {
                        name: "Foo".to_string(),
                        fields: vec![],
                    },
                    StructDefinition {
                        name: "Bar".to_string(),
                        fields: vec![],
                    },
                ],
                enums: vec![],
            },
        );
    }

    #[test]
    fn structs_can_reference_each_other() {
        // happy path
        expect_ast(
            "struct Foo {} struct Bar { baz: Foo }",
            &FileNode {
                structs: vec![
                    StructDefinition {
                        name: "Foo".to_string(),
                        fields: vec![],
                    },
                    StructDefinition {
                        name: "Bar".to_string(),
                        fields: vec![Field {
                            name: "baz".to_string(),
                            t: Type::Named("Foo".to_string()),
                        }],
                    },
                ],
                enums: vec![],
            },
        );

        // recursive
        expect_ast(
            "struct Foo { bar: Foo }",
            &FileNode {
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![Field {
                        name: "bar".to_string(),
                        t: Type::Named("Foo".to_string()),
                    }],
                }],
                enums: vec![],
            },
        );

        // undefined reference
        expect_error(
            "struct Foo { bar: Bar }",
            &Error::Message("Reference to undefined type: Bar".to_string()),
        );
    }

    #[test]
    fn parses_enums() {
        expect_ast(
            "enum Foo { bar, baz }",
            &FileNode {
                structs: vec![],
                enums: vec![EnumDefinition {
                    name: "Foo".to_string(),
                    variants: vec![
                        Variant {
                            name: "bar".to_string(),
                            t: None,
                        },
                        Variant {
                            name: "baz".to_string(),
                            t: None,
                        },
                    ],
                }],
            },
        );

        // commas are necessary between variants
        expect_error(
            "enum Foo { bar baz }",
            &Error::WrongToken {
                expected: TokenKind::Comma,
                got: TokenKind::Identifier("baz".to_string()),
            },
        );
    }

    #[test]
    fn parses_enums_with_associated_types() {
        expect_ast(
            "enum Foo { bar(Int), baz(String) }",
            &FileNode {
                structs: vec![],
                enums: vec![EnumDefinition {
                    name: "Foo".to_string(),
                    variants: vec![
                        Variant {
                            name: "bar".to_string(),
                            t: Some(Type::Primitive(Primitive::Int)),
                        },
                        Variant {
                            name: "baz".to_string(),
                            t: Some(Type::Primitive(Primitive::String)),
                        },
                    ],
                }],
            },
        );

        expect_ast(
            "enum Foo {bar(Int?), baz([String]), qux([[[Int?]?]])}",
            &FileNode {
                structs: vec![],
                enums: vec![EnumDefinition {
                    name: "Foo".to_string(),
                    variants: vec![
                        Variant {
                            name: "bar".to_string(),
                            t: Some(Type::Optional(Box::new(Type::Primitive(Primitive::Int)))),
                        },
                        Variant {
                            name: "baz".to_string(),
                            t: Some(Type::Array(Box::new(Type::Primitive(Primitive::String)))),
                        },
                        Variant {
                            name: "qux".to_string(),
                            t: Some(Type::Array(Box::new(Type::Array(Box::new(
                                Type::Optional(Box::new(Type::Array(Box::new(Type::Optional(
                                    Box::new(Type::Primitive(Primitive::Int)),
                                ))))),
                            ))))),
                        },
                    ],
                }],
            },
        );
    }

    #[test]
    fn parses_multiple_enums() {
        expect_ast(
            "enum Foo {} enum Bar {}",
            &FileNode {
                structs: vec![],
                enums: vec![
                    EnumDefinition {
                        name: "Foo".to_string(),
                        variants: vec![],
                    },
                    EnumDefinition {
                        name: "Bar".to_string(),
                        variants: vec![],
                    },
                ],
            },
        );
    }

    #[test]
    fn enums_can_reference_each_other() {
        // happy path
        expect_ast(
            "enum Foo {} enum Bar { baz(Foo) }",
            &FileNode {
                structs: vec![],
                enums: vec![
                    EnumDefinition {
                        name: "Foo".to_string(),
                        variants: vec![],
                    },
                    EnumDefinition {
                        name: "Bar".to_string(),
                        variants: vec![Variant {
                            name: "baz".to_string(),
                            t: Some(Type::Named("Foo".to_string())),
                        }],
                    },
                ],
            },
        );

        // recursive
        expect_ast(
            "enum Foo { bar(Foo) }",
            &FileNode {
                structs: vec![],
                enums: vec![EnumDefinition {
                    name: "Foo".to_string(),
                    variants: vec![Variant {
                        name: "bar".to_string(),
                        t: Some(Type::Named("Foo".to_string())),
                    }],
                }],
            },
        );

        // undefined reference
        expect_error(
            "enum Foo { bar(Bar) }",
            &Error::Message("Reference to undefined type: Bar".to_string()),
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn structs_and_enums_together() {
        expect_ast(
            "struct Foo {} enum Bar { baz(Foo) }",
            &FileNode {
                structs: vec![StructDefinition {
                    name: "Foo".to_string(),
                    fields: vec![],
                }],
                enums: vec![EnumDefinition {
                    name: "Bar".to_string(),
                    variants: vec![Variant {
                        name: "baz".to_string(),
                        t: Some(Type::Named("Foo".to_string())),
                    }],
                }],
            },
        );

        // empty file
        expect_ast(
            "",
            &FileNode {
                structs: vec![],
                enums: vec![],
            },
        );

        // complicated mess
        expect_ast(
            r"
            enum Something {
              one,
              two,
              three(Int)
            }

            struct Person {
              id: UUID,
              name: String,
              birthday: Date,
              hat_brand: String?,
              children: [Person],
              something_else: [[[Something?]?]]
            }

            enum Foo {
              bar([Person]?),
              baz(Foo)
            }",
            &FileNode {
                structs: vec![StructDefinition {
                    name: "Person".to_string(),
                    fields: vec![
                        Field {
                            name: "id".to_string(),
                            t: Type::Primitive(Primitive::Uuid),
                        },
                        Field {
                            name: "name".to_string(),
                            t: Type::Primitive(Primitive::String),
                        },
                        Field {
                            name: "birthday".to_string(),
                            t: Type::Primitive(Primitive::Date),
                        },
                        Field {
                            name: "hat_brand".to_string(),
                            t: Type::Optional(Box::new(Type::Primitive(Primitive::String))),
                        },
                        Field {
                            name: "children".to_string(),
                            t: Type::Array(Box::new(Type::Named("Person".to_string()))),
                        },
                        Field {
                            name: "something_else".to_string(),
                            t: Type::Array(Box::new(Type::Array(Box::new(Type::Optional(
                                Box::new(Type::Array(Box::new(Type::Optional(Box::new(
                                    Type::Named("Something".to_string()),
                                ))))),
                            ))))),
                        },
                    ],
                }],
                enums: vec![
                    EnumDefinition {
                        name: "Something".to_string(),
                        variants: vec![
                            Variant {
                                name: "one".to_string(),
                                t: None,
                            },
                            Variant {
                                name: "two".to_string(),
                                t: None,
                            },
                            Variant {
                                name: "three".to_string(),
                                t: Some(Type::Primitive(Primitive::Int)),
                            },
                        ],
                    },
                    EnumDefinition {
                        name: "Foo".to_string(),
                        variants: vec![
                            Variant {
                                name: "bar".to_string(),
                                t: Some(Type::Optional(Box::new(Type::Array(Box::new(
                                    Type::Named("Person".to_string()),
                                ))))),
                            },
                            Variant {
                                name: "baz".to_string(),
                                t: Some(Type::Named("Foo".to_string())),
                            },
                        ],
                    },
                ],
            },
        );
    }
}
