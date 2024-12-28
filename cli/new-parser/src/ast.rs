#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Primitive {
    Int,
    Float,
    String,
    Bool,
    Date,
    Uuid,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Named(String),
    Optional(Box<Type>),
    Array(Box<Type>),
    Primitive(Primitive),
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructDefinition {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub t: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumDefinition {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Variant {
    pub name: String,
    pub t: Option<Type>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IOType {
    Type(Type),
    Struct(StructDefinition),
    Enum(EnumDefinition),
}
