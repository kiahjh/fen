// Created by Fen v0.5.0 at 16:09:11 on 2025-02-14
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func enumsWithAssociatedValuesTest() async throws -> Response<EnumsWithAssociatedValuesTestOutput> {
    return try await self.fetcher.get(from: "/_fen_/enums-with-associated-values-test", sessionToken: nil)
  }
}

enum EnumsWithAssociatedValuesTestOutput: Codable, Equatable {
  case first_option(Int)
  case second_option([String])
}