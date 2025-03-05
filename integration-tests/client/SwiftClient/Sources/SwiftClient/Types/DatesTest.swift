// Created by Fen v0.5.2 at 13:10:09 on 2025-03-05
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  func datesTest(input: Date) async throws -> Response<Date> {
    return try await self.fetcher.post(
      to: "/_fen_/dates-test",
      with: input,
      returning: Date.self,
      sessionToken: nil
    )
  }
}