// Created by Fen v0.5.0 at 16:09:11 on 2025-02-14
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