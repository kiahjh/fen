import Foundation

struct Fetcher {
  var endpoint: String

  let jsonEncoder = JSONEncoder()
  let jsonDecoder = JSONDecoder()

  func get<T>(from path: String) async throws -> Response<T> where T: Decodable {
    let url = URL(string: self.endpoint + path)!
    let (data, _) = try await URLSession.shared.data(from: url)

    let tag = try self.jsonDecoder.decode(ResponseType.self, from: data)
    if tag.type == "success" {
      let response = try self.jsonDecoder.decode(SuccessResponse<T>.self, from: data)
      return .success(SuccessResponse(data: response.data))
    } else {
      let response = try self.jsonDecoder.decode(FailureResponse.self, from: data)
      return .failure(FailureResponse(message: response.message, status: response.status))
    }
  }

  func post<T: Decodable>(
    to path: String,
    with body: Encodable,
    returning type: T.Type
  ) async throws -> Response<T> {
    let url = URL(string: self.endpoint + path)!
    var request = URLRequest(url: url)
    request.httpMethod = "POST"
    request.setValue("application/json", forHTTPHeaderField: "Content-Type")

    let body = try self.jsonEncoder.encode(body)
    request.httpBody = body

    let (data, _) = try await URLSession.shared.data(for: request)

    let tag = try self.jsonDecoder.decode(ResponseType.self, from: data)
    if tag.type == "success" {
      let response = try self.jsonDecoder.decode(SuccessResponse<T>.self, from: data)
      return .success(SuccessResponse(data: response.data))
    } else {
      let response = try self.jsonDecoder.decode(FailureResponse.self, from: data)
      return .failure(FailureResponse(message: response.message, status: response.status))
    }
  }
}

struct ResponseType: Decodable {
  var type: String
}

enum Response<T: Decodable> {
  case success(SuccessResponse<T>)
  case failure(FailureResponse)
}

struct SuccessResponse<T: Decodable>: Decodable {
  let data: T
}

struct FailureResponse: Decodable {
  let message: String
  let status: Int
}
