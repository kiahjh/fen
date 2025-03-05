// Created by Fen v0.5.2 at 13:10:09 on 2025-03-05
// Do not manually modify this file as it is automatically generated

extension APIClient {
  func floatsTest(input: Double) async throws -> Response<Double> {
    return try await self.fetcher.post(
      to: "/_fen_/floats-test",
      with: input,
      returning: Double.self,
      sessionToken: nil
    )
  }
}