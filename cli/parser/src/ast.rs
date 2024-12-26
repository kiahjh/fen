// file
#[derive(Debug, PartialEq)]
pub struct FileNode {
    pub structs: Vec<StructDefinition>,
    pub enums: Vec<EnumDefinition>,
}

// primitives
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Primitive {
    Int,
    Float,
    String,
    Bool,
    Date,
    Uuid,
}

// types
#[derive(Debug, PartialEq)]
pub enum Type {
    Named(String),
    Optional(Box<Type>),
    Array(Box<Type>),
    Primitive(Primitive),
}

// structs
#[derive(Debug, PartialEq)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub t: Type,
}

// enums
#[derive(Debug, PartialEq)]
pub struct EnumDefinition {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub name: String,
    pub t: Option<Type>,
}
