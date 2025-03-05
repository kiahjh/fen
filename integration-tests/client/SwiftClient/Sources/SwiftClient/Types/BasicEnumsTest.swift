// Created by Fen v0.5.3 at 14:49:02 on 2025-03-05
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

  private enum CodingKeys: String, CodingKey {
    case type
  }

  private enum BasicEnumsTestInputType: String, Codable {
    case happy
    case sad
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(BasicEnumsTestInputType.self, forKey: .type)

    switch type {
    case .happy:
      self = .happy
    case .sad:
      self = .sad
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .happy:
      try container.encode(BasicEnumsTestInputType.happy, forKey: .type)
    case .sad:
      try container.encode(BasicEnumsTestInputType.sad, forKey: .type)
    }
  }
}

enum BasicEnumsTestOutput: Codable, Equatable {
  case foo
  case bar
  case baz

  private enum CodingKeys: String, CodingKey {
    case type
  }

  private enum BasicEnumsTestOutputType: String, Codable {
    case foo
    case bar
    case baz
  }

  init(from decoder: Decoder) throws {
    let container = try decoder.container(keyedBy: CodingKeys.self)
    let type = try container.decode(BasicEnumsTestOutputType.self, forKey: .type)

    switch type {
    case .foo:
      self = .foo
    case .bar:
      self = .bar
    case .baz:
      self = .baz
    }
  }

  func encode(to encoder: Encoder) throws {
    var container = encoder.container(keyedBy: CodingKeys.self)

    switch self {
    case .foo:
      try container.encode(BasicEnumsTestOutputType.foo, forKey: .type)
    case .bar:
      try container.encode(BasicEnumsTestOutputType.bar, forKey: .type)
    case .baz:
      try container.encode(BasicEnumsTestOutputType.baz, forKey: .type)
    }
  }
}