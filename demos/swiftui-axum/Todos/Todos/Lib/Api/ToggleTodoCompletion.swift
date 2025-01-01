// Created by Fen v0.1.0 at 14:03:38 on 2025-01-01
// Do not manually modify this file as it is automatically generated

import Foundation

extension ApiClient {
  /// Completes or uncompletes a todo
  func toggleTodoCompletion(input: UUID, sessionToken: String) async throws -> Response<NoData> {
    return try await self.fetcher.post(
      to: "/toggle-todo-completion",
      with: Input(payload: input),
      returning: NoData.self,
      sessionToken: sessionToken
    )
  }
}