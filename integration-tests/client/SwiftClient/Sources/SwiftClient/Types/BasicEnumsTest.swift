// Created by Fen v0.5.0 at 16:09:11 on 2025-02-14
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func basicEnumsTest(input: BasicEnumsTestInput) async throws -> Response<BasicEnumsTestOutput> {
    return try await self.fetcher.post(
      to: "/_fen_/basic-enums-test",
      with: input,
      returning: BasicEnumsTestOutput.self,
      sessionToken: nil
    )
  }
}

enum BasicEnumsTestInput: Codable, Equatable {
  case happy
  case sad
}

enum BasicEnumsTestOutput: Codable, Equatable {
  case foo
  case bar
  case baz
}