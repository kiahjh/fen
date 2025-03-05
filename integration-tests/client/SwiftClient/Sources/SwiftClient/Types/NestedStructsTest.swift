// Created by Fen v0.5.1 at 21:15:42 on 2025-03-04
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  func nestedStructsTest() async throws -> Response<Human> {
    return try await self.fetcher.get(from: "/_fen_/nested-structs-test", sessionToken: nil)
  }
}

struct Human: Codable, Equatable {
  var name: String
  var birthday: Date
  var vehicle: Vehicle
}

struct Vehicle: Codable, Equatable {
  var color: String
  var year: Int
}