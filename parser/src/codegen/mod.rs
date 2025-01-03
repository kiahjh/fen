use crate::ast::{
    EnumDefinition, Field, FileNode, IOType, Primitive, StructDefinition, Type, Variant,
};
use name_transforms::{pascal_to_camel, pascal_to_kebab, snake_to_camel, snake_to_pascal};

pub mod name_transforms;

pub enum Codeability {
    Encodable,
    Decodable,
}

pub struct Context {
    pub override_name: Option<String>,
    pub codeability: Option<Codeability>,
}

pub trait GenCode {
    // client
    fn swift_client_code(&self, ctx: &Context) -> String;

    // server
    fn rust_server_code(&self, _ctx: &Context) -> String {
        unimplemented!() // TODO: remove this (should be required)
    }
}

impl GenCode for FileNode {
    #[allow(clippy::too_many_lines)]
    fn swift_client_code(&self, ctx: &Context) -> String {
        // helpers:
        let return_type_name = self.output.as_ref().map_or_else(
            || "NoData".to_string(),
            |output| match output {
                IOType::Type(t) => t.swift_client_code(ctx),
                IOType::Struct(_) | IOType::Enum(_) => self.name.clone() + "Output",
            },
        );

        // open the extension
        let mut lines = vec!["extension ApiClient {".to_string()];

        // add documentation
        if let Some(description) = &self.description {
            lines.push(format!("  /// {description}"));
        }

        // declare the function
        let mut func_decl = format!("  func {}(", pascal_to_camel(&self.name));

        // function arguments (derived from input)
        if let Some(input) = &self.input {
            match input {
                IOType::Type(t) => {
                    func_decl.push_str(&format!("input: {}", t.swift_client_code(ctx)));
                }
                IOType::Enum(_) => {
                    func_decl.push_str(&format!("input: {}", self.name.clone() + "Input"));
                }
                IOType::Struct(s) => {
                    let mut args_str = vec![];
                    for Field { name, t } in &s.fields {
                        args_str.push(format!("{}: {}", name, t.swift_client_code(ctx)));
                    }
                    func_decl.push_str(&args_str.join(", "));
                }
            }
        }

        // require session token if route is authed
        if self.authed && self.input.is_some() {
            func_decl.push_str(", sessionToken: String");
        } else if self.authed {
            func_decl.push_str("sessionToken: String");
        }

        func_decl.push_str(") async throws -> Response<");

        // function return type (derived from output)
        func_decl.push_str(format!("{return_type_name}> {{").as_str());

        // add the function declaration to the lines
        lines.push(func_decl);

        // return statement
        lines.push(format!(
            "    return try await self.fetcher.{}",
            if self.input.is_some() {
                "post(".to_string()
            } else {
                format!(
                    "get(from: \"/{}\"{})",
                    pascal_to_kebab(&self.name),
                    if self.authed {
                        ", sessionToken: sessionToken"
                    } else {
                        ""
                    }
                )
            }
        ));

        // return statement body (for post requests)
        if self.input.is_some() {
            // add the path
            lines.push(format!("      to: \"/{}\",", pascal_to_kebab(&self.name)));

            // add the input
            let input_payload = match &self.input.as_ref().unwrap() {
                IOType::Type(_) | IOType::Enum(_) => "input".to_string(),
                IOType::Struct(s) => {
                    let mut pairs = vec![];
                    for field in &s.fields {
                        pairs.push(format!("{}: {}", field.name, field.name));
                    }
                    format!("{}({})", self.name.clone() + "Input", pairs.join(", "))
                }
            };
            lines.push(format!("      with: Input(payload: {input_payload}),"));

            // add the return type
            lines.push(format!(
                "      returning: {}.self{}",
                return_type_name,
                if self.authed { "," } else { "" }
            ));

            // add the session token
            if self.authed {
                lines.push("      sessionToken: sessionToken".to_string());
            }

            lines.push("    )".to_string());
        }

        lines.push("  }".to_string());
        lines.push("}".to_string());

        // if input is a struct or enum, define it
        if let Some(IOType::Struct(s)) = &self.input {
            lines.push(String::new());
            lines.push(s.swift_client_code(&Context {
                override_name: Some(self.name.clone() + "Input"),
                codeability: Some(Codeability::Encodable),
            }));
        } else if let Some(IOType::Enum(e)) = &self.input {
            lines.push(String::new());
            lines.push(e.swift_client_code(&Context {
                override_name: Some(self.name.clone() + "Input"),
                codeability: Some(Codeability::Encodable),
            }));
        }

        // if output is a struct or enum, define it
        if let Some(IOType::Struct(s)) = &self.output {
            lines.push(String::new());
            lines.push(s.swift_client_code(&Context {
                override_name: Some(self.name.clone() + "Output"),
                codeability: Some(Codeability::Decodable),
            }));
        } else if let Some(IOType::Enum(e)) = &self.output {
            lines.push(String::new());
            lines.push(e.swift_client_code(&Context {
                override_name: Some(self.name.clone() + "Output"),
                codeability: Some(Codeability::Decodable),
            }));
        }

        // generate definitions for helper structs
        for struct_def in &self.structs {
            lines.push(String::new());
            lines.push(struct_def.swift_client_code(&Context {
                override_name: None,
                codeability: Some(Codeability::Decodable),
            }));
        }

        // generate definitions for helper enums
        for enum_def in &self.enums {
            lines.push(String::new());
            lines.push(enum_def.swift_client_code(&Context {
                override_name: None,
                codeability: Some(Codeability::Decodable),
            }));
        }

        let code = lines.join("\n");
        if code.contains("Date") || code.contains("UUID") {
            "import Foundation\n\n".to_string() + &code
        } else {
            code
        }
    }

    #[allow(clippy::too_many_lines)]
    fn rust_server_code(&self, ctx: &Context) -> String {
        let mut lines: Vec<String> = vec![];

        if self.input.is_some() {
            let input = self.input.as_ref().unwrap();
            match input {
                IOType::Type(t) => lines.push(format!(
                    "pub type {} = {};",
                    "Input",
                    t.rust_server_code(ctx)
                )),
                IOType::Struct(s) => lines.push(s.rust_server_code(&Context {
                    override_name: Some("Input".to_string()),
                    codeability: None,
                })),
                IOType::Enum(e) => lines.push(e.rust_server_code(&Context {
                    override_name: Some("Input".to_string()),
                    codeability: None,
                })),
            }
        }

        if self.input.is_some() && self.output.is_some() {
            lines.push(String::new());
        }

        if self.output.is_some() {
            let output = self.output.as_ref().unwrap();
            match output {
                IOType::Type(t) => lines.push(format!(
                    "pub type {} = {};",
                    "Output",
                    t.rust_server_code(ctx)
                )),
                IOType::Struct(s) => lines.push(s.rust_server_code(&Context {
                    override_name: Some("Output".to_string()),
                    codeability: None,
                })),
                IOType::Enum(e) => lines.push(e.rust_server_code(&Context {
                    override_name: Some("Output".to_string()),
                    codeability: None,
                })),
            }
        }

        for struct_def in &self.structs {
            lines.push(String::new());
            lines.push(struct_def.rust_server_code(ctx));
        }

        for enum_def in &self.enums {
            lines.push(String::new());
            lines.push(enum_def.rust_server_code(ctx));
        }

        let mut code = lines.join("\n");

        if code.contains("Uuid")
            || code.contains("DateTime<Utc>")
            || code.contains("Deserialize")
            || code.contains("Serialize")
        {
            code = "\n".to_string() + &code;
        }
        if code.contains("Uuid") {
            code = "use uuid::Uuid;\n".to_string() + &code;
        }
        if code.contains("Serialize") && code.contains("Deserialize") {
            code = "use serde::{Deserialize, Serialize};\n".to_string() + &code;
        } else if code.contains("Serialize") {
            code = "use serde::Serialize;\n".to_string() + &code;
        } else if code.contains("Deserialize") {
            code = "use serde::Deserialize;\n".to_string() + &code;
        }
        if code.contains("DateTime<Utc>") {
            code = "use chrono::{DateTime, Utc};\n".to_string() + &code;
        }

        code
    }
}

impl GenCode for StructDefinition {
    fn swift_client_code(&self, ctx: &Context) -> String {
        let mut lines = vec![];
        lines.push(format!(
            "struct {}{} {{",
            ctx.override_name.as_ref().map_or(&self.name, |n| n),
            ctx.codeability.as_ref().map_or("", |c| match c {
                Codeability::Encodable => ": Encodable",
                Codeability::Decodable => ": Decodable",
            })
        ));
        for field in &self.fields {
            lines.push(field.swift_client_code(ctx));
        }
        lines.push("}".to_string());
        lines.join("\n")
    }

    fn rust_server_code(&self, ctx: &Context) -> String {
        let mut lines = vec![];

        lines.push("#[derive(Serialize, Deserialize)]".to_string());
        lines.push("#[serde(rename_all = \"camelCase\")]".to_string());
        lines.push(format!(
            "pub struct {} {{",
            ctx.override_name.as_ref().map_or(&self.name, |name| name)
        ));
        for field in &self.fields {
            lines.push(field.rust_server_code(ctx));
        }
        lines.push("}".to_string());

        lines.join("\n")
    }
}

impl GenCode for Field {
    fn swift_client_code(&self, ctx: &Context) -> String {
        format!(
            "  var {}: {}",
            snake_to_camel(&self.name),
            self.t.swift_client_code(ctx)
        )
    }

    fn rust_server_code(&self, ctx: &Context) -> String {
        format!("    pub {}: {},", &self.name, self.t.rust_server_code(ctx))
    }
}

impl GenCode for EnumDefinition {
    fn swift_client_code(&self, ctx: &Context) -> String {
        let mut lines = vec![];

        lines.push(format!(
            "enum {}{} {{",
            ctx.override_name.as_ref().map_or(&self.name, |n| n),
            ctx.codeability.as_ref().map_or("", |c| match c {
                Codeability::Encodable => ": Encodable",
                Codeability::Decodable => ": Decodable",
            })
        ));
        for variant in &self.variants {
            lines.push(variant.swift_client_code(ctx));
        }
        lines.push("}".to_string());

        lines.join("\n")
    }

    fn rust_server_code(&self, ctx: &Context) -> String {
        let mut lines = vec![];

        lines.push("#[derive(Serialize, Deserialize)]".to_string());
        lines.push("#[serde(tag = \"type\", rename_all = \"camelCase\")]".to_string());
        lines.push(format!(
            "pub enum {} {{",
            ctx.override_name.as_ref().map_or(&self.name, |n| n)
        ));
        for variant in &self.variants {
            lines.push(variant.rust_server_code(ctx));
        }
        lines.push("}".to_string());

        lines.join("\n")
    }
}

impl GenCode for Variant {
    fn swift_client_code(&self, ctx: &Context) -> String {
        format!(
            "  case {}{}",
            self.name,
            self.t
                .as_ref()
                .map_or_else(String::new, |t| format!("({})", t.swift_client_code(ctx)))
        )
    }

    fn rust_server_code(&self, ctx: &Context) -> String {
        format!(
            "    {}{},",
            snake_to_pascal(&self.name),
            self.t
                .as_ref()
                .map_or_else(String::new, |t| format!("({})", t.rust_server_code(ctx)))
        )
    }
}

impl GenCode for Type {
    fn swift_client_code(&self, ctx: &Context) -> String {
        match &self {
            Self::Named(n) => n.clone(),
            Self::Optional(t) => format!("{}?", t.swift_client_code(ctx)),
            Self::Array(t) => format!("[{}]", t.swift_client_code(ctx)),
            Self::Primitive(p) => p.swift_client_code(ctx),
        }
    }

    fn rust_server_code(&self, ctx: &Context) -> String {
        match &self {
            Self::Named(n) => n.clone(),
            Self::Optional(t) => format!("Option<{}>", t.rust_server_code(ctx)),
            Self::Array(t) => format!("Vec<{}>", t.rust_server_code(ctx)),
            Self::Primitive(p) => p.rust_server_code(ctx),
        }
    }
}

impl GenCode for Primitive {
    fn swift_client_code(&self, _ctx: &Context) -> String {
        match &self {
            Self::Int => "Int".to_string(),
            Self::Float => "Double".to_string(),
            Self::String => "String".to_string(),
            Self::Bool => "Bool".to_string(),
            Self::Date => "Date".to_string(),
            Self::Uuid => "UUID".to_string(),
        }
    }

    fn rust_server_code(&self, _ctx: &Context) -> String {
        match &self {
            Self::Int => "isize".to_string(),
            Self::Float => "f64".to_string(),
            Self::String => "String".to_string(),
            Self::Bool => "bool".to_string(),
            Self::Date => "DateTime<Utc>".to_string(),
            Self::Uuid => "Uuid".to_string(),
        }
    }
}

mod swift_client_tests {
    use super::{Context, GenCode};
    use crate::Parser;
    use pretty_assertions::assert_eq;

    fn expect_swift(fen_code: &str, swift_code: &str) {
        let mut parser = Parser::new(fen_code);
        let ast = parser.parse().unwrap();
        let swift = ast.swift_client_code(&Context {
            override_name: None,
            codeability: None,
        });
        assert_eq!(swift, swift_code);
    }

    #[test]
    fn just_output() {
        expect_swift(
            r#"
name: "GetTodos"
description: "Fetches all todos"
authed: true

---

@output [Todo]

---

Todo {
  id: UUID
  name: String
  description: String?
  due: Date?
  is_completed: Bool
}
            "#
            .trim(),
            r#"
import Foundation

extension ApiClient {
  /// Fetches all todos
  func getTodos(sessionToken: String) async throws -> Response<[Todo]> {
    return try await self.fetcher.get(from: "/get-todos", sessionToken: sessionToken)
  }
}

struct Todo: Decodable {
  var id: UUID
  var name: String
  var description: String?
  var due: Date?
  var isCompleted: Bool
}
            "#
            .trim(),
        );
    }

    #[test]
    fn just_input() {
        expect_swift(
            r#"
name: "ToggleTodoCompletion"
description: "Completes or uncompletes a todo"
authed: true

---

@input UUID
            "#
            .trim(),
            r#"
import Foundation

extension ApiClient {
  /// Completes or uncompletes a todo
  func toggleTodoCompletion(input: UUID, sessionToken: String) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/toggle-todo-completion",
      with: Input(payload: input),
      returning: NoData.self,
      sessionToken: sessionToken
    )
  }
}
            "#
            .trim(),
        );
    }

    #[test]
    fn struct_for_input_and_output() {
        expect_swift(
            r#"
name: "Test"

---

@input {
  id: UUID
  foo: String
  bar: [Date]?
}

@output {
  stuff: [Thing]
}

---

Thing {
  type: ThingType
  happy: Bool
}

ThingType (
  a
  b
  c
)
            "#
            .trim(),
            r#"
import Foundation

extension ApiClient {
  func test(id: UUID, foo: String, bar: [Date]?) async throws -> Response<TestOutput> {
    return try await self.fetcher.post(
      to: "/test",
      with: Input(payload: TestInput(id: id, foo: foo, bar: bar)),
      returning: TestOutput.self
    )
  }
}

struct TestInput: Encodable {
  var id: UUID
  var foo: String
  var bar: [Date]?
}

struct TestOutput: Decodable {
  var stuff: [Thing]
}

struct Thing: Decodable {
  var type: ThingType
  var happy: Bool
}

enum ThingType: Decodable {
  case a
  case b
  case c
}
            "#
            .trim(),
        );
    }

    #[test]
    fn struct_for_input_no_output() {
        expect_swift(
            r#"
name: "Test"
authed: true

---

@input {
  id: UUID
  foo: String
  bar: [Date]?
}
            "#
            .trim(),
            r#"
import Foundation

extension ApiClient {
  func test(id: UUID, foo: String, bar: [Date]?, sessionToken: String) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/test",
      with: Input(payload: TestInput(id: id, foo: foo, bar: bar)),
      returning: NoData.self,
      sessionToken: sessionToken
    )
  }
}

struct TestInput: Encodable {
  var id: UUID
  var foo: String
  var bar: [Date]?
}
            "#
            .trim(),
        );
    }

    #[test]
    fn input_is_struct_output_is_type() {
        expect_swift(
            r#"
name: "YetAnotherTest"

---

@input {
  id: UUID
  foo: String
}

@output [UUID]
            "#
            .trim(),
            r#"
import Foundation

extension ApiClient {
  func yetAnotherTest(id: UUID, foo: String) async throws -> Response<[UUID]> {
    return try await self.fetcher.post(
      to: "/yet-another-test",
      with: Input(payload: YetAnotherTestInput(id: id, foo: foo)),
      returning: [UUID].self
    )
  }
}

struct YetAnotherTestInput: Encodable {
  var id: UUID
  var foo: String
}
            "#
            .trim(),
        );
    }

    #[test]
    fn without_foundation() {
        expect_swift(
            r#"
name: "Test"
authed: true

---

@output Int
            "#
            .trim(),
            r#"
extension ApiClient {
  func test(sessionToken: String) async throws -> Response<Int> {
    return try await self.fetcher.get(from: "/test", sessionToken: sessionToken)
  }
}
            "#
            .trim(),
        );
    }

    #[test]
    fn enum_output_with_helpers() {
        expect_swift(
            r#"
name: "EnumTest"
description: "Just testing out enums"
authed: true

---

@output (
  single
  married(Spouse)
)

---

Spouse {
  name: String
  age: Int
  has_beard: Bool
  ocupation: Job
}

Job (
  developer
  construction
  other(String?)
)
            "#
            .trim(),
            r#"
extension ApiClient {
  /// Just testing out enums
  func enumTest(sessionToken: String) async throws -> Response<EnumTestOutput> {
    return try await self.fetcher.get(from: "/enum-test", sessionToken: sessionToken)
  }
}

enum EnumTestOutput: Decodable {
  case single
  case married(Spouse)
}

struct Spouse: Decodable {
  var name: String
  var age: Int
  var hasBeard: Bool
  var ocupation: Job
}

enum Job: Decodable {
  case developer
  case construction
  case other(String?)
}
            "#
            .trim(),
        );
    }

    #[test]
    fn enum_input() {
        expect_swift(
            r#"
name: "AnotherEnumTest"
description: "Just testing out some more enums"

---

@input (
  a
  b(Int)
)
            "#
            .trim(),
            r#"
extension ApiClient {
  /// Just testing out some more enums
  func anotherEnumTest(input: AnotherEnumTestInput) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/another-enum-test",
      with: Input(payload: input),
      returning: NoData.self
    )
  }
}

enum AnotherEnumTestInput: Encodable {
  case a
  case b(Int)
}
            "#
            .trim(),
        );
    }
}

mod rust_server_tests {
    use super::{Context, GenCode};
    use crate::Parser;
    use pretty_assertions::assert_eq;

    fn expect_rust(fen_code: &str, rust_code: &str) {
        let mut parser = Parser::new(fen_code);
        let ast = parser.parse().unwrap();
        let rust = ast.rust_server_code(&Context {
            override_name: None,
            codeability: None,
        });
        assert_eq!(rust, rust_code);
    }

    #[test]
    fn just_output() {
        expect_rust(
            r#"
name: "GetTodos"
description: "Fetches all todos"
authed: true

---

@output [Todo]

---

Todo {
  id: UUID
  name: String
  description: String?
  due: Date?
  is_completed: Bool
}
            "#
            .trim(),
            r#"
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Output = Vec<Todo>;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub due: Option<DateTime<Utc>>,
    pub is_completed: bool,
}
            "#
            .trim(),
        );
    }

    #[test]
    fn just_input() {
        expect_rust(
            r#"
name: "ToggleTodoCompletion"
description: "Completes or uncompletes a todo"
authed: true

---

@input UUID
            "#
            .trim(),
            r"
use uuid::Uuid;

pub type Input = Uuid;
                "
            .trim(),
        );
    }

    #[test]
    fn struct_for_input_and_output() {
        expect_rust(
            r#"
name: "Test"

---

@input {
  id: UUID
  foo: String
  bar: [Date]?
}

@output {
  stuff: [Thing]
}

---

Thing {
  type: ThingType
  happy: Bool
}

ThingType (
  a
  b
  c
)
                "#
            .trim(),
            r#"
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub id: Uuid,
    pub foo: String,
    pub bar: Option<Vec<DateTime<Utc>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub stuff: Vec<Thing>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thing {
    pub type: ThingType,
    pub happy: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ThingType {
    A,
    B,
    C,
}
                "#
            .trim(),
        );
    }

    #[test]
    fn struct_for_input_no_output() {
        expect_rust(
            r#"
name: "Test"
authed: true

---

@input {
  id: UUID
  foo: String
  bar: [Date]?
}
                "#
            .trim(),
            r#"
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub id: Uuid,
    pub foo: String,
    pub bar: Option<Vec<DateTime<Utc>>>,
}
             "#
            .trim(),
        );
    }

    #[test]
    fn input_is_struct_output_is_type() {
        expect_rust(
            r#"
name: "YetAnotherTest"

---

@input {
  id: UUID
  foo: String
}

@output [UUID]
            "#
            .trim(),
            r#"
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub id: Uuid,
    pub foo: String,
}

pub type Output = Vec<Uuid>;
            "#
            .trim(),
        );
    }

    #[test]
    fn without_foundation() {
        expect_rust(
            r#"
name: "Test"
authed: true

---

@output Int
                    "#
            .trim(),
            r"
pub type Output = isize;
                "
            .trim(),
        );
    }

    #[test]
    fn enum_output_with_helpers() {
        expect_rust(
            r#"
name: "EnumTest"
description: "Just testing out enums"
authed: true

---

@output (
  single
  married(Spouse)
)

---

Spouse {
  name: String
  age: Int
  has_beard: Bool
  ocupation: Job
}

Job (
  developer
  construction
  other(String?)
)
                "#
            .trim(),
            r#"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Output {
    Single,
    Married(Spouse),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spouse {
    pub name: String,
    pub age: isize,
    pub has_beard: bool,
    pub ocupation: Job,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Job {
    Developer,
    Construction,
    Other(Option<String>),
}
                "#
            .trim(),
        );
    }

    #[test]
    fn enum_input() {
        expect_rust(
            r#"
name: "AnotherEnumTest"
description: "Just testing out some more enums"

---

@input (
  a
  b(Int)
)
            "#
            .trim(),
            r#"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Input {
    A,
    B(isize),
}
            "#
            .trim(),
        );
    }
}
