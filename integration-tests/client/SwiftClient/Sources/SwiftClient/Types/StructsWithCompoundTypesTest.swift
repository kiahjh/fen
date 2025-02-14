// Created by Fen v0.5.0 at 16:09:11 on 2025-02-14
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