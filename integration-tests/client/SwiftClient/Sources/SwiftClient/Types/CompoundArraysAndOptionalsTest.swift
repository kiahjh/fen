// Created by Fen v0.5.3 at 14:49:02 on 2025-03-05
// Do not manually modify this file as it is automatically generated

import Foundation

extension APIClient {
  func compoundArraysAndOptionalsTest(input: [[Int]?]) async throws -> Response<[UUID]?> {
    return try await self.fetcher.post(
      to: "/_fen_/compound-arrays-and-optionals-test",
      with: input,
      returning: [UUID]?.self,
      sessionToken: nil
    )
  }
}