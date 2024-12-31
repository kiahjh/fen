let api = ApiClient(fetcher: Fetcher(endpoint: "{{API_ENDPOINT}}"))

struct ApiClient {
  var fetcher: Fetcher
}
