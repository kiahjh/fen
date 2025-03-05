// Created by Fen v0.5.2 at 13:10:09 on 2025-03-05
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  func uuidsTest(input: UUID) async throws -> Response<UUID> {
    return try await self.fetcher.post(
      to: "/_fen_/uuids-test",
      with: input,
      returning: UUID.self,
      sessionToken: nil
    )
  }
}