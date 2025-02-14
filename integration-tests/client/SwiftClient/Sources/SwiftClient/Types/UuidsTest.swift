// Created by Fen v0.5.0 at 16:09:11 on 2025-02-14
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