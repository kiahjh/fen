import Foundation

struct APIClient {
  var fetcher: any Fetcher
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

  let jsonEncoder = JSONEncoder()
  let jsonDecoder = JSONDecoder()

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

    let tag = try self.jsonDecoder.decode(ResponseType.self, from: data)
    if tag.type == "success" {
      let response = try self.jsonDecoder.decode(SuccessResponse<T>.self, from: data)
      return .success(response.data)
    } else {
      let response = try self.jsonDecoder.decode(FailureResponse.self, from: data)
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

    let body = try self.jsonEncoder.encode(body)
    request.httpBody = body

    let (data, _) = try await URLSession.shared.data(for: request)

    let tag = try self.jsonDecoder.decode(ResponseType.self, from: data)
    if tag.type == "success" {
      let response = try self.jsonDecoder.decode(SuccessResponse<T>.self, from: data)
      return .success(response.data)
    } else {
      let response = try self.jsonDecoder.decode(FailureResponse.self, from: data)
      return .failure(message: response.message, status: response.status)
    }
  }
}

struct NoData: Decodable {}

struct ResponseType: Decodable {
  var type: String
}

enum Response<T: Decodable & Sendable>: Sendable {
  case success(T)
  case failure(message: String, status: Int)
}

struct SuccessResponse<T: Decodable & Sendable>: Decodable, Sendable {
  let data: T
}

struct FailureResponse: Decodable {
  let message: String
  let status: Int
}
