// Created by Fen v0.1.0 at 11:12:15 on 2025-01-01
// Do not manually modify this file as it is automatically generated

extension ApiClient {
  func getTodos() async throws -> Response<[Todo]> {
    return try await self.fetcher.get(from: "/get-todos")
  }
}

struct Todo: Decodable {
  var id: UUID
  var name: String
  var description: String?
  var due: Date?
  var isCompleted: Bool
}