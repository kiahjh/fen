// Created by Fen v0.5.0 at 16:59:07 on 2025-03-04
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