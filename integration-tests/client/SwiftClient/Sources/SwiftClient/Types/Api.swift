// Created by Fen v0.5.2 at 13:10:09 on 2025-03-05
// Do not manually modify this file as it is automatically generated

import Foundation

#if canImport(FoundationNetworking)
  import FoundationNetworking
#endif

struct APIClient {
  var fetcher: any Fetcher

  static func encodeAsData(_ value: Encodable) throws -> Data {
    let encoder = JSONEncoder()
    encoder.dateEncodingStrategy = .iso8601
    return try encoder.encode(value)
  }

  static func encodeAsString(_ value: Encodable) throws -> String {
    let data = try self.encodeAsData(value)
    return String(data: data, encoding: .utf8)!
  }

  static func decode<T: Decodable>(_ data: Data, type: T.Type) throws -> T {
    let decoder = JSONDecoder()
    decoder.dateDecodingStrategy = .iso8601
    return try decoder.decode(T.self, from: data)
  }

  static func decode<T: Decodable>(_ string: String, type: T.Type) throws -> T {
    let data = string.data(using: .utf8)!
    return try self.decode(data, type: T.self)
  }
}

protocol Fetcher: Sendable {
  func get<T>(from path: String, sessionToken: String?) async throws -> Response<T>
  func post<T: Decodable, U: Encodable>(
    to path: String,
    with body: U,
    returning type: T.Type,
    sessionToken: String?
  ) async throws -> Response<T>
}

struct LiveFetcher: Fetcher {
  var endpoint: String

  func get<T>(from path: String, sessionToken: String?) async throws -> Response<T>
  where T: Decodable {
    let url = URL(string: self.endpoint + path)!
    var request = URLRequest(url: url)
    request.httpMethod = "GET"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    if let sessionToken = sessionToken {
      request.setValue("Bearer \(sessionToken)", forHTTPHeaderField: "Authorization")
    }

    let (data, _) = try await URLSession.shared.data(for: request)
    let tag = try APIClient.decode(data, type: ResponseType.self)

    if tag.type == "success" {
      let response = try APIClient.decode(data, type: SuccessResponse<T>.self)
      return .success(response.value)
    } else {
      let response = try APIClient.decode(data, type: FailureResponse.self)
      return .failure(message: response.message, status: response.status)
    }
  }

  func post<T: Decodable, U: Encodable>(
    to path: String,
    with body: U,
    returning type: T.Type,
    sessionToken: String? = nil
  ) async throws -> Response<T> {
    let url = URL(string: self.endpoint + path)!
    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")
    if let sessionToken = sessionToken {
      request.setValue("Bearer \(sessionToken)", forHTTPHeaderField: "Authorization")
    }

    let body = try APIClient.encodeAsData(body)
    request.httpBody = body

    let (data, _) = try await URLSession.shared.data(for: request)

    let tag = try APIClient.decode(data, type: ResponseType.self)
    if tag.type == "success" {
      let response = try APIClient.decode(data, type: SuccessResponse<T>.self)
      return .success(response.value)
    } else {
      let response = try APIClient.decode(data, type: FailureResponse.self)
      return .failure(message: response.message, status: response.status)
    }
  }
}

struct NoData: Decodable {}

struct ResponseType: Decodable {
  var type: String
}

enum Response<T: Decodable & Sendable>: Decodable, Sendable {
  case success(T)
  case failure(message: String, status: Int)
}

struct SuccessResponse<T: Decodable & Sendable>: Decodable, Sendable {
  let value: T
}

struct FailureResponse: Decodable {
  let message: String
  let status: Int
}
