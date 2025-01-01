use name_transforms::{pascal_to_camel, pascal_to_kebab, snake_to_camel};

use crate::ast::{EnumDefinition, Field, FileNode, IOType, Primitive, StructDefinition, Type};

mod name_transforms;

pub trait GenCode {
    fn swift_client_code(&self) -> String;
}

impl GenCode for FileNode {
    #[allow(clippy::too_many_lines)]
    fn swift_client_code(&self) -> String {
        // helpers:
        let return_type_name = self.output.as_ref().map_or_else(
            || "NoData".to_string(),
            |output| match output {
                IOType::Type(t) => t.swift_client_code(),
                IOType::Struct(_) => self.name.clone() + "Output",
                IOType::Enum(_) => todo!(),
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
                    func_decl.push_str(&format!(
                        "input: {}) async throws -> Response<",
                        t.swift_client_code()
                    ));
                }
                IOType::Struct(s) => {
                    let mut args_str = vec![];
                    for Field { name, t } in &s.fields {
                        args_str.push(format!("{}: {}", name, t.swift_client_code()));
                    }
                    func_decl.push_str(&args_str.join(", "));
                    func_decl.push_str(") async throws -> Response<");
                }
                IOType::Enum(_) => todo!(),
            }
        } else {
            func_decl.push_str(") async throws -> Response<");
        }

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
                format!("get(from: \"/{}\")", pascal_to_kebab(&self.name))
            }
        ));

        // return statement body (for post requests)
        if self.input.is_some() {
            // add the path
            lines.push(format!("      to: \"/{}\",", pascal_to_kebab(&self.name)));

            // add the input
            let input_payload = match &self.input.as_ref().unwrap() {
                IOType::Type(_) => "input".to_string(),
                IOType::Struct(s) => {
                    let mut pairs = vec![];
                    for field in &s.fields {
                        pairs.push(format!("{}: {}", field.name, field.name));
                    }
                    format!("{}({})", self.name.clone() + "Input", pairs.join(", "))
                }
                IOType::Enum(_) => todo!(),
            };
            lines.push(format!("      with: Input(payload: {input_payload}),"));

            // add the return type
            lines.push(format!("      returning: {return_type_name}.self"));

            lines.push("    )".to_string());
        }

        lines.push("  }".to_string());
        lines.push("}".to_string());

        // if input is a struct, define it
        if let Some(IOType::Struct(s)) = &self.input {
            lines.push(String::new());
            lines.push(format!("struct {}Input: Encodable {{", self.name.clone()));
            for field in &s.fields {
                lines.push(format!(
                    "  var {}: {}",
                    field.name,
                    field.t.swift_client_code()
                ));
            }
            lines.push("}".to_string());
        }

        // if output is a struct, define it
        if let Some(IOType::Struct(s)) = &self.output {
            lines.push(String::new());
            lines.push(format!("struct {}Output: Decodable {{", self.name.clone()));
            for field in &s.fields {
                lines.push(format!(
                    "  var {}: {}",
                    field.name,
                    field.t.swift_client_code()
                ));
            }
            lines.push("}".to_string());
        }

        // generate definitions for helper structs
        for struct_def in &self.structs {
            lines.push(String::new());
            lines.push(struct_def.swift_client_code());
        }

        // generate definitions for helper enums
        for enum_def in &self.enums {
            lines.push(String::new());
            lines.push(enum_def.swift_client_code());
        }

        let code = lines.join("\n");
        if code.contains("Date") || code.contains("UUID") {
            "import Foundation\n\n".to_string() + &code
        } else {
            code
        }
    }
}

impl GenCode for StructDefinition {
    fn swift_client_code(&self) -> String {
        let mut lines = vec![];
        lines.push(format!("struct {}: Decodable {{", self.name));
        for field in &self.fields {
            lines.push(format!(
                "  var {}: {}",
                snake_to_camel(&field.name),
                field.t.swift_client_code()
            ));
        }
        lines.push("}".to_string());
        lines.join("\n")
    }
}

impl GenCode for EnumDefinition {
    fn swift_client_code(&self) -> String {
        let mut lines = vec![];
        lines.push(format!("enum {}: Decodable {{", self.name));
        for variant in &self.variants {
            lines.push(format!("  case {}", variant.name));
        }
        lines.push("}".to_string());
        lines.join("\n")
    }
}

impl GenCode for Type {
    fn swift_client_code(&self) -> String {
        match &self {
            Self::Named(n) => n.clone(),
            Self::Optional(t) => format!("{}?", t.swift_client_code()),
            Self::Array(t) => format!("[{}]", t.swift_client_code()),
            Self::Primitive(p) => p.swift_client_code(),
        }
    }
}

impl GenCode for Primitive {
    fn swift_client_code(&self) -> String {
        match &self {
            Self::Int => "Int".to_string(),
            Self::Float => "Double".to_string(),
            Self::String => "String".to_string(),
            Self::Bool => "Bool".to_string(),
            Self::Date => "Date".to_string(),
            Self::Uuid => "UUID".to_string(),
        }
    }
}

mod swift_client_tests {
    use super::GenCode;
    use crate::Parser;
    use pretty_assertions::assert_eq;

    fn expect_swift(fen_code: &str, swift_code: &str) {
        let mut parser = Parser::new(fen_code);
        let ast = parser.parse().unwrap();
        let swift = ast.swift_client_code();
        assert_eq!(swift, swift_code);
    }

    #[test]
    fn just_output() {
        expect_swift(
            r#"
name: "GetTodos"
description: "Fetches all todos"

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
  func getTodos() async throws -> Response<[Todo]> {
    return try await self.fetcher.get(from: "/get-todos")
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
  func toggleTodoCompletion(input: UUID) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/toggle-todo-completion",
      with: Input(payload: input),
      returning: NoData.self
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
  func test(id: UUID, foo: String, bar: [Date]?) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/test",
      with: Input(payload: TestInput(id: id, foo: foo, bar: bar)),
      returning: NoData.self
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

---

@output Int
            "#
            .trim(),
            r#"
extension ApiClient {
  func test() async throws -> Response<Int> {
    return try await self.fetcher.get(from: "/test")
  }
}
            "#
            .trim(),
        );
    }
}
