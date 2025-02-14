// Created by Fen v0.5.0 at 16:09:11 on 2025-02-14
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  func composingStructsAndEnumsTest() async throws -> Response<Person> {
    return try await self.fetcher.get(from: "/_fen_/composing-structs-and-enums-test", sessionToken: nil)
  }
}

struct Person: Codable, Equatable, Identifiable {
  var name: String
  var birthday: Date
  var id: UUID
  var car: Car
}

struct Car: Codable, Equatable {
  var color: String
  var gear: Gear
}

enum Gear: Codable, Equatable {
  case park
  case neutral
  case reverse
  case drive(Speed)
}

enum Speed: Codable, Equatable {
  case first
  case second
  case third
  case fourth
  case fifth
}