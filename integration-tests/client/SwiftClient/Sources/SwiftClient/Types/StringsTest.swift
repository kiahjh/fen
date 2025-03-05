// Created by Fen v0.5.3 at 14:49:02 on 2025-03-05
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func stringsTest(input: String) async throws -> Response<String> {
    return try await self.fetcher.post(
      to: "/_fen_/strings-test",
      with: input,
      returning: String.self,
      sessionToken: nil
    )
  }
}