use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct FileNode {
    // metadata
    pub name: String,
    pub description: Option<String>,
    pub authed: bool,

    // i/o
    pub input: Option<IOType>,
    pub output: IOType,

    // helper types
    pub structs: Vec<StructDefinition>,
    pub enums: Vec<EnumDefinition>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
#[serde(tag = "type")]
pub enum Primitive {
    Int,
    Float,
    String,
    Bool,
    Date,
    Uuid,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum Type {
    Named(String),
    Optional(Box<Type>),
    Array(Box<Type>),
    Primitive(Primitive),
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub t: Type,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct EnumDefinition {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Variant {
    pub name: String,
    #[serde(rename = "type")]
    pub t: Option<Type>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum IOType {
    Type(Type),
    Struct(StructDefinition),
    Enum(EnumDefinition),
}
