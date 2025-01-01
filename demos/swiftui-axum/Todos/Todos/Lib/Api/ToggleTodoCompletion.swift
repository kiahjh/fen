// Created by Fen v0.1.0 at 11:12:15 on 2025-01-01
// Do not manually modify this file as it is automatically generated

extension ApiClient {
  func toggleTodoCompletion(input: UUID) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/toggle-todo-completion",
      with: Input(payload: input),
      returning: NoData.self
    )
  }
}