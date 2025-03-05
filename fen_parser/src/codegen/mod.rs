use crate::ast::{
    EnumDefinition, Field, FileNode, IOType, Primitive, StructDefinition, Type, Variant,
};
use name_transforms::{
    pascal_to_camel, pascal_to_kebab, pascal_to_snake, snake_to_camel, snake_to_pascal,
};

pub mod name_transforms;

pub struct Context {
    pub override_name: Option<String>,
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
        let mut lines = vec!["extension APIClient {".to_string()];

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
                    "get(from: \"/_fen_/{}\"{})",
                    pascal_to_kebab(&self.name),
                    if self.authed {
                        ", sessionToken: sessionToken"
                    } else {
                        ", sessionToken: nil"
                    }
                )
            }
        ));

        // return statement body (for post requests)
        if self.input.is_some() {
            // add the path
            lines.push(format!(
                "      to: \"/_fen_/{}\",",
                pascal_to_kebab(&self.name)
            ));

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
            lines.push(format!("      with: {input_payload},"));

            // add the return type
            lines.push(format!("      returning: {return_type_name}.self,"));

            // add the session token
            if self.authed {
                lines.push("      sessionToken: sessionToken".to_string());
            } else {
                lines.push("      sessionToken: nil".to_string());
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
            }));
        } else if let Some(IOType::Enum(e)) = &self.input {
            lines.push(String::new());
            lines.push(e.swift_client_code(&Context {
                override_name: Some(self.name.clone() + "Input"),
            }));
        }

        // if output is a struct or enum, define it
        if let Some(IOType::Struct(s)) = &self.output {
            lines.push(String::new());
            lines.push(s.swift_client_code(&Context {
                override_name: Some(self.name.clone() + "Output"),
            }));
        } else if let Some(IOType::Enum(e)) = &self.output {
            lines.push(String::new());
            lines.push(e.swift_client_code(&Context {
                override_name: Some(self.name.clone() + "Output"),
            }));
        }

        // generate definitions for helper structs
        for struct_def in &self.structs {
            lines.push(String::new());
            lines.push(struct_def.swift_client_code(&Context {
                override_name: None,
            }));
        }

        // generate definitions for helper enums
        for enum_def in &self.enums {
            lines.push(String::new());
            lines.push(enum_def.swift_client_code(&Context {
                override_name: None,
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
                })),
                IOType::Enum(e) => lines.push(e.rust_server_code(&Context {
                    override_name: Some("Input".to_string()),
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
                })),
                IOType::Enum(e) => lines.push(e.rust_server_code(&Context {
                    override_name: Some("Output".to_string()),
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
            "struct {}: Codable, Equatable{} {{",
            ctx.override_name.as_ref().map_or(&self.name, |n| n),
            if self.fields.iter().any(|f| f.name == "id") {
                ", Identifiable"
            } else {
                ""
            }
        ));
        for field in &self.fields {
            lines.push(field.swift_client_code(ctx));
        }

        // if any of the fields are Type::Optional:
        if self.fields.iter().any(|f| matches!(f.t, Type::Optional(_))) {
            // coding keys
            lines.push(String::new());
            lines.push("  private enum CodingKeys: String, CodingKey {".to_string());
            for field in &self.fields {
                lines.push(format!("    case {}", snake_to_camel(&field.name)));
            }
            lines.push("  }".to_string());

            // encode
            lines.push(String::new());
            lines.push("  func encode(to encoder: Encoder) throws {".to_string());
            lines.push(
                "    var container = encoder.container(keyedBy: CodingKeys.self)".to_string(),
            );
            lines.push(String::new());
            for field in &self.fields {
                if let Type::Optional(t) = &field.t {
                    lines.push(format!(
                        "    switch self.{} {{",
                        snake_to_camel(&field.name)
                    ));
                    lines.push("    case let .some(value):".to_string());
                    lines.push(format!(
                        "      try container.encode(value, forKey: .{})",
                        snake_to_camel(&field.name)
                    ));
                    lines.push("    case .none:".to_string());
                    lines.push(format!(
                        "      try container.encodeNil(forKey: .{})",
                        snake_to_camel(&field.name)
                    ));
                    lines.push("    }".to_string());
                } else {
                    lines.push(format!(
                        "    try container.encode(self.{}, forKey: .{})",
                        snake_to_camel(&field.name),
                        snake_to_camel(&field.name)
                    ));
                }
            }
            lines.push("  }".to_string());
        }

        lines.push("}".to_string());
        lines.join("\n")
    }

    fn rust_server_code(&self, ctx: &Context) -> String {
        let mut lines = vec![];

        lines.push("#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]".to_string());
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

        let enum_name = ctx.override_name.as_ref().map_or(&self.name, |n| n);

        lines.push(format!("enum {enum_name}: Codable, Equatable {{",));
        for variant in &self.variants {
            lines.push(variant.swift_client_code(ctx));
        }

        // coding keys
        lines.push(String::new());
        lines.push("  private enum CodingKeys: String, CodingKey {".to_string());
        lines.push("    case type".to_string());
        if self.variants.iter().any(|v| v.t.is_some()) {
            lines.push("    case value".to_string());
        }
        lines.push("  }".to_string());

        // key types
        let key_types_name = format!("{enum_name}Type");
        lines.push(String::new());
        lines.push(format!(
            "  private enum {key_types_name}: String, Codable {{",
        ));
        for variant in &self.variants {
            lines.push(format!("    case {}", snake_to_camel(&variant.name)));
        }
        lines.push("  }".to_string());

        // init from decoder
        lines.push(String::new());
        lines.push("  init(from decoder: Decoder) throws {".to_string());
        lines.push(
            "    let container = try decoder.container(keyedBy: CodingKeys.self)".to_string(),
        );
        lines.push(format!(
            "    let type = try container.decode({key_types_name}.self, forKey: .type)"
        ));
        lines.push(String::new());
        lines.push("    switch type {".to_string());
        for variant in &self.variants {
            lines.push(format!("    case .{}:", snake_to_camel(&variant.name)));
            match &variant.t {
                Some(Type::Optional(t)) => {
                    lines.push(format!(
                        "      let value = try container.decodeIfPresent({}.self, forKey: .value)",
                        t.swift_client_code(ctx)
                    ));
                    lines.push(format!(
                        "      self = .{}(value)",
                        snake_to_camel(&variant.name)
                    ));
                }
                Some(other) => {
                    lines.push(format!(
                        "      let value = try container.decode({}.self, forKey: .value)",
                        other.swift_client_code(ctx)
                    ));
                    lines.push(format!(
                        "      self = .{}(value)",
                        snake_to_camel(&variant.name)
                    ));
                }
                None => {
                    lines.push(format!("      self = .{}", snake_to_camel(&variant.name)));
                }
            };
        }
        lines.push("    }".to_string());
        lines.push("  }".to_string());

        // encode
        lines.push(String::new());
        lines.push("  func encode(to encoder: Encoder) throws {".to_string());
        lines.push("    var container = encoder.container(keyedBy: CodingKeys.self)".to_string());
        lines.push(String::new());
        lines.push("    switch self {".to_string());
        for variant in &self.variants {
            if let Some(_) = &variant.t {
                lines.push(format!(
                    "    case .{}(let value):",
                    snake_to_camel(&variant.name)
                ));
                lines.push(format!(
                    "      try container.encode({}.{}, forKey: .type)",
                    key_types_name,
                    snake_to_camel(&variant.name)
                ));
                lines.push("      try container.encode(value, forKey: .value)".to_string());
            } else {
                lines.push(format!("    case .{}:", snake_to_camel(&variant.name)));
                lines.push(format!(
                    "      try container.encode({}.{}, forKey: .type)",
                    key_types_name,
                    snake_to_camel(&variant.name)
                ));
            }
        }
        lines.push("    }".to_string());
        lines.push("  }".to_string());

        lines.push("}".to_string());
        lines.join("\n")
    }

    fn rust_server_code(&self, ctx: &Context) -> String {
        let mut lines = vec![];

        lines.push(format!(
            "#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq{})]",
            if self.annotations.is_empty() {
                ""
            } else {
                ", sqlx::Type"
            }
        ));
        lines.push(format!(
            "#[serde(tag = \"type\", {}rename_all = \"camelCase\")]",
            if self.variants.iter().any(|v| v.t.is_some()) {
                "content = \"value\", "
            } else {
                ""
            }
        ));
        if self.annotations.contains(&"sqlxType".to_string()) {
            lines.push(format!(
                "#[sqlx(type_name = \"{}\", rename_all = \"SCREAMING_SNAKE_CASE\")]",
                pascal_to_snake(&self.name)
            ));
        }
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
            snake_to_camel(&self.name),
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

extension APIClient {
  /// Fetches all todos
  func getTodos(sessionToken: String) async throws -> Response<[Todo]> {
    return try await self.fetcher.get(from: "/_fen_/get-todos", sessionToken: sessionToken)
  }
}

struct Todo: Codable, Equatable, Identifiable {
  var id: UUID
  var name: String
  var description: String?
  var due: Date?
  var isCompleted: Bool

  private enum CodingKeys: String, CodingKey {
    case id
    case name
    case description
    case due
    case isCompleted
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    try container.encode(self.id, forKey: .id)
    try container.encode(self.name, forKey: .name)
    switch self.description {
    case let .some(value):
      try container.encode(value, forKey: .description)
    case .none:
      try container.encodeNil(forKey: .description)
    }
    switch self.due {
    case let .some(value):
      try container.encode(value, forKey: .due)
    case .none:
      try container.encodeNil(forKey: .due)
    }
    try container.encode(self.isCompleted, forKey: .isCompleted)
  }
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

extension APIClient {
  /// Completes or uncompletes a todo
  func toggleTodoCompletion(input: UUID, sessionToken: String) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/_fen_/toggle-todo-completion",
      with: input,
      returning: NoData.self,
      sessionToken: sessionToken
    )
  }
}
            "#
            .trim(),
        );
    }

    #[allow(clippy::too_many_lines)]
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
  first_option
  second_option
  third_option
)
            "#
            .trim(),
            r#"
import Foundation

extension APIClient {
  func test(id: UUID, foo: String, bar: [Date]?) async throws -> Response<TestOutput> {
    return try await self.fetcher.post(
      to: "/_fen_/test",
      with: TestInput(id: id, foo: foo, bar: bar),
      returning: TestOutput.self,
      sessionToken: nil
    )
  }
}

struct TestInput: Codable, Equatable, Identifiable {
  var id: UUID
  var foo: String
  var bar: [Date]?

  private enum CodingKeys: String, CodingKey {
    case id
    case foo
    case bar
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    try container.encode(self.id, forKey: .id)
    try container.encode(self.foo, forKey: .foo)
    switch self.bar {
    case let .some(value):
      try container.encode(value, forKey: .bar)
    case .none:
      try container.encodeNil(forKey: .bar)
    }
  }
}

struct TestOutput: Codable, Equatable {
  var stuff: [Thing]
}

struct Thing: Codable, Equatable {
  var type: ThingType
  var happy: Bool
}

enum ThingType: Codable, Equatable {
  case firstOption
  case secondOption
  case thirdOption

  private enum CodingKeys: String, CodingKey {
    case type
  }

  private enum ThingTypeType: String, Codable {
    case firstOption
    case secondOption
    case thirdOption
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(ThingTypeType.self, forKey: .type)

    switch type {
    case .firstOption:
      self = .firstOption
    case .secondOption:
      self = .secondOption
    case .thirdOption:
      self = .thirdOption
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .firstOption:
      try container.encode(ThingTypeType.firstOption, forKey: .type)
    case .secondOption:
      try container.encode(ThingTypeType.secondOption, forKey: .type)
    case .thirdOption:
      try container.encode(ThingTypeType.thirdOption, forKey: .type)
    }
  }
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

extension APIClient {
  func test(id: UUID, foo: String, bar: [Date]?, sessionToken: String) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/_fen_/test",
      with: TestInput(id: id, foo: foo, bar: bar),
      returning: NoData.self,
      sessionToken: sessionToken
    )
  }
}

struct TestInput: Codable, Equatable, Identifiable {
  var id: UUID
  var foo: String
  var bar: [Date]?

  private enum CodingKeys: String, CodingKey {
    case id
    case foo
    case bar
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    try container.encode(self.id, forKey: .id)
    try container.encode(self.foo, forKey: .foo)
    switch self.bar {
    case let .some(value):
      try container.encode(value, forKey: .bar)
    case .none:
      try container.encodeNil(forKey: .bar)
    }
  }
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

extension APIClient {
  func yetAnotherTest(id: UUID, foo: String) async throws -> Response<[UUID]> {
    return try await self.fetcher.post(
      to: "/_fen_/yet-another-test",
      with: YetAnotherTestInput(id: id, foo: foo),
      returning: [UUID].self,
      sessionToken: nil
    )
  }
}

struct YetAnotherTestInput: Codable, Equatable, Identifiable {
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
extension APIClient {
  func test(sessionToken: String) async throws -> Response<Int> {
    return try await self.fetcher.get(from: "/_fen_/test", sessionToken: sessionToken)
  }
}
            "#
            .trim(),
        );
    }

    #[allow(clippy::too_many_lines)]
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
extension APIClient {
  /// Just testing out enums
  func enumTest(sessionToken: String) async throws -> Response<EnumTestOutput> {
    return try await self.fetcher.get(from: "/_fen_/enum-test", sessionToken: sessionToken)
  }
}

enum EnumTestOutput: Codable, Equatable {
  case single
  case married(Spouse)

  private enum CodingKeys: String, CodingKey {
    case type
    case value
  }

  private enum EnumTestOutputType: String, Codable {
    case single
    case married
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(EnumTestOutputType.self, forKey: .type)

    switch type {
    case .single:
      self = .single
    case .married:
      let value = try container.decode(Spouse.self, forKey: .value)
      self = .married(value)
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .single:
      try container.encode(EnumTestOutputType.single, forKey: .type)
    case .married(let value):
      try container.encode(EnumTestOutputType.married, forKey: .type)
      try container.encode(value, forKey: .value)
    }
  }
}

struct Spouse: Codable, Equatable {
  var name: String
  var age: Int
  var hasBeard: Bool
  var ocupation: Job
}

enum Job: Codable, Equatable {
  case developer
  case construction
  case other(String?)

  private enum CodingKeys: String, CodingKey {
    case type
    case value
  }

  private enum JobType: String, Codable {
    case developer
    case construction
    case other
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(JobType.self, forKey: .type)

    switch type {
    case .developer:
      self = .developer
    case .construction:
      self = .construction
    case .other:
      let value = try container.decodeIfPresent(String.self, forKey: .value)
      self = .other(value)
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .developer:
      try container.encode(JobType.developer, forKey: .type)
    case .construction:
      try container.encode(JobType.construction, forKey: .type)
    case .other(let value):
      try container.encode(JobType.other, forKey: .type)
      try container.encode(value, forKey: .value)
    }
  }
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
extension APIClient {
  /// Just testing out some more enums
  func anotherEnumTest(input: AnotherEnumTestInput) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/_fen_/another-enum-test",
      with: input,
      returning: NoData.self,
      sessionToken: nil
    )
  }
}

enum AnotherEnumTestInput: Codable, Equatable {
  case a
  case b(Int)

  private enum CodingKeys: String, CodingKey {
    case type
    case value
  }

  private enum AnotherEnumTestInputType: String, Codable {
    case a
    case b
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(AnotherEnumTestInputType.self, forKey: .type)

    switch type {
    case .a:
      self = .a
    case .b:
      let value = try container.decode(Int.self, forKey: .value)
      self = .b(value)
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .a:
      try container.encode(AnotherEnumTestInputType.a, forKey: .type)
    case .b(let value):
      try container.encode(AnotherEnumTestInputType.b, forKey: .type)
      try container.encode(value, forKey: .value)
    }
  }
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub id: Uuid,
    pub foo: String,
    pub bar: Option<Vec<DateTime<Utc>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub stuff: Vec<Thing>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Thing {
    pub type: ThingType,
    pub happy: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Output {
    Single,
    Married(Spouse),
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Spouse {
    pub name: String,
    pub age: isize,
    pub has_beard: bool,
    pub ocupation: Job,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Input {
    A,
    B(isize),
}
            "#
            .trim(),
        );
    }

    #[test]
    fn sqlx_types() {
        expect_rust(
            r#"
name: "GetRepertoire"
description: "Get a user's repertoire"
authed: true

---

@output [Song]

---

Song {
  id: UUID
  title: String
  familiarity: FamiliarityLevel
}

@sqlxType
FamiliarityLevel (
  todo
  learning
  playable
  good
  mastered
)
                "#
            .trim(),
            r#"
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Output = Vec<Song>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub familiarity: FamiliarityLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, sqlx::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
#[sqlx(type_name = "familiarity_level", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FamiliarityLevel {
    Todo,
    Learning,
    Playable,
    Good,
    Mastered,
}
                "#
            .trim(),
        );
    }
}
