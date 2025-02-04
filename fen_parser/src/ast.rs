use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct FileNode {
    // metadata
    pub name: String,
    pub description: Option<String>,
    pub authed: bool,

    // i/o
    pub input: Option<IOType>,
    pub output: Option<IOType>,

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

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub enum Type {
    Named(String),
    Optional(Box<Type>),
    Array(Box<Type>),
    Primitive(Primitive),
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<Field>,
    pub annotations: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub t: Type,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct EnumDefinition {
    pub name: String,
    pub variants: Vec<Variant>,
    pub annotations: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct Variant {
    pub name: String,
    #[serde(rename = "type")]
    pub t: Option<Type>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub enum IOType {
    Type(Type),
    Struct(StructDefinition),
    Enum(EnumDefinition),
}
