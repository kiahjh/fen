// Created by Fen v0.5.1 at 21:15:42 on 2025-03-04
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func intsTest(input: Int) async throws -> Response<Int> {
    return try await self.fetcher.post(
      to: "/_fen_/ints-test",
      with: input,
      returning: Int.self,
      sessionToken: nil
    )
  }
}