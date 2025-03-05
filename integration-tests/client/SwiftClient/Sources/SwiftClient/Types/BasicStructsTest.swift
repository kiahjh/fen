// Created by Fen v0.5.1 at 21:15:42 on 2025-03-04
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  /// Get a person by their id
  func basicStructsTest(id: UUID) async throws -> Response<BasicStructsTestOutput> {
    return try await self.fetcher.post(
      to: "/_fen_/basic-structs-test",
      with: BasicStructsTestInput(id: id),
      returning: BasicStructsTestOutput.self,
      sessionToken: nil
    )
  }
}

struct BasicStructsTestInput: Codable, Equatable, Identifiable {
  var id: UUID
}

struct BasicStructsTestOutput: Codable, Equatable {
  var name: String
  var age: Int
  var birthday: Date
  var hasBeard: Bool
}