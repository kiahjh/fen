// Created by Fen v0.5.0 at 16:59:07 on 2025-03-04
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

  private enum CodingKeys: String, CodingKey {
    case type
    case value
  }

  private enum GearType: String, Codable {
    case park
    case neutral
    case reverse
    case drive
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(GearType.self, forKey: .type)

    switch type {
    case .park:
      self = .park
    case .neutral:
      self = .neutral
    case .reverse:
      self = .reverse
    case .drive:
      let value = try container.decode(Speed.self, forKey: .value)
      self = .drive(value)
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .park:
      try container.encode(GearType.park, forKey: .type)
    case .neutral:
      try container.encode(GearType.neutral, forKey: .type)
    case .reverse:
      try container.encode(GearType.reverse, forKey: .type)
    case .drive(let value):
      try container.encode(GearType.drive, forKey: .type)
      try container.encode(value, forKey: .value)
    }
  }
}

enum Speed: Codable, Equatable {
  case first
  case second
  case third
  case fourth
  case fifth

  private enum CodingKeys: String, CodingKey {
    case type
  }

  private enum SpeedType: String, Codable {
    case first
    case second
    case third
    case fourth
    case fifth
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(SpeedType.self, forKey: .type)

    switch type {
    case .first:
      self = .first
    case .second:
      self = .second
    case .third:
      self = .third
    case .fourth:
      self = .fourth
    case .fifth:
      self = .fifth
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .first:
      try container.encode(SpeedType.first, forKey: .type)
    case .second:
      try container.encode(SpeedType.second, forKey: .type)
    case .third:
      try container.encode(SpeedType.third, forKey: .type)
    case .fourth:
      try container.encode(SpeedType.fourth, forKey: .type)
    case .fifth:
      try container.encode(SpeedType.fifth, forKey: .type)
    }
  }
}