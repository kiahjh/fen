// Created by Fen v0.5.1 at 21:15:42 on 2025-03-04
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