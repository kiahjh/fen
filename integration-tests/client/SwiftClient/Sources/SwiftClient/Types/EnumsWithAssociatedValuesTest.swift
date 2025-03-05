// Created by Fen v0.5.3 at 14:49:02 on 2025-03-05
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func enumsWithAssociatedValuesTest() async throws -> Response<EnumsWithAssociatedValuesTestOutput> {
    return try await self.fetcher.get(from: "/_fen_/enums-with-associated-values-test", sessionToken: nil)
  }
}

enum EnumsWithAssociatedValuesTestOutput: Codable, Equatable {
  case firstOption(Int)
  case secondOption([String])

  private enum CodingKeys: String, CodingKey {
    case type
    case value
  }

  private enum EnumsWithAssociatedValuesTestOutputType: String, Codable {
    case firstOption
    case secondOption
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(EnumsWithAssociatedValuesTestOutputType.self, forKey: .type)

    switch type {
    case .firstOption:
      let value = try container.decode(Int.self, forKey: .value)
      self = .firstOption(value)
    case .secondOption:
      let value = try container.decode([String].self, forKey: .value)
      self = .secondOption(value)
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .firstOption(let value):
      try container.encode(EnumsWithAssociatedValuesTestOutputType.firstOption, forKey: .type)
      try container.encode(value, forKey: .value)
    case .secondOption(let value):
      try container.encode(EnumsWithAssociatedValuesTestOutputType.secondOption, forKey: .type)
      try container.encode(value, forKey: .value)
    }
  }
}