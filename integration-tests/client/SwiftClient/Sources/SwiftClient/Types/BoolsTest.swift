// Created by Fen v0.5.3 at 14:49:02 on 2025-03-05
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func boolsTest(input: Bool) async throws -> Response<Bool> {
    return try await self.fetcher.post(
      to: "/_fen_/bools-test",
      with: input,
      returning: Bool.self,
      sessionToken: nil
    )
  }
}