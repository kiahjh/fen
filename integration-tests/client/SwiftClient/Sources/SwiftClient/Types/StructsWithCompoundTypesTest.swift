// Created by Fen v0.5.3 at 14:49:02 on 2025-03-05
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func structsWithCompoundTypesTest(foo: String?) async throws -> Response<StructsWithCompoundTypesTestOutput> {
    return try await self.fetcher.post(
      to: "/_fen_/structs-with-compound-types-test",
      with: StructsWithCompoundTypesTestInput(foo: foo),
      returning: StructsWithCompoundTypesTestOutput.self,
      sessionToken: nil
    )
  }
}

struct StructsWithCompoundTypesTestInput: Codable, Equatable {
  var foo: String?

  private enum CodingKeys: String, CodingKey {
    case foo
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self.foo {
    case let .some(value):
      try container.encode(value, forKey: .foo)
    case .none:
      try container.encodeNil(forKey: .foo)
    }
  }
}

struct StructsWithCompoundTypesTestOutput: Codable, Equatable {
  var bar: [Int?]
}