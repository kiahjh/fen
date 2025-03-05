// Created by Fen v0.5.2 at 13:10:09 on 2025-03-05
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  func arrayOfStructsWithDateTest() async throws -> Response<[Song]> {
    return try await self.fetcher.get(from: "/_fen_/array-of-structs-with-date-test", sessionToken: nil)
  }
}

struct Song: Codable, Equatable {
  var title: String
  var composed: Date
}