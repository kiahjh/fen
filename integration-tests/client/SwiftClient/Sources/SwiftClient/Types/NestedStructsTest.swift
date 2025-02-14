// Created by Fen v0.5.0 at 16:09:11 on 2025-02-14
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  func nestedStructsTest() async throws -> Response<Person> {
    return try await self.fetcher.get(from: "/_fen_/nested-structs-test", sessionToken: nil)
  }
}

struct Person: Codable, Equatable {
  var name: String
  var birthday: Date
  var car: Car
}

struct Car: Codable, Equatable {
  var color: String
  var year: Int
}