// Created by Fen v0.5.0 at 16:09:11 on 2025-02-14
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  func optionalsTest(input: Int?) async throws -> Response<Date?> {
    return try await self.fetcher.post(
      to: "/_fen_/optionals-test",
      with: input,
      returning: Date?.self,
      sessionToken: nil
    )
  }
}