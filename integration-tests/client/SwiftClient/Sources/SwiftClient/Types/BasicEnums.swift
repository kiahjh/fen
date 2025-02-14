// Created by Fen v0.5.0 at 14:59:37 on 2025-02-14
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func basicEnums(input: BasicEnumsInput) async throws -> Response<BasicEnumsOutput> {
    return try await self.fetcher.post(
      to: "/_fen_/basic-enums",
      with: input,
      returning: BasicEnumsOutput.self,
      sessionToken: nil
    )
  }
}

enum BasicEnumsInput: Codable, Equatable {
  case happy
  case sad
}

enum BasicEnumsOutput: Codable, Equatable {
  case foo
  case bar
  case baz
}