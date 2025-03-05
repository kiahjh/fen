// Created by Fen v0.5.1 at 21:15:42 on 2025-03-04
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
}

struct StructsWithCompoundTypesTestOutput: Codable, Equatable {
  var bar: [Int?]
}