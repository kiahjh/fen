# Changelog

## [0.4.0]
- swift: rename `ApiClient` -> `APIClient`
- swift: made `Fetcher` into a protocol and expose a `LiveFetcher` implementation (for testing purposes)
- swift: flattened `Response` type (no more `SuccessResponse` and `FailureResponse`)

## [0.3.1]
- swift: make generated structs and enums `Equatable`

## [0.3.0]
- fen: support for comments with `//`

## [0.2.0]
- all: appended `_fen_/` to all paths
- all: groundwork laid for annotation support
- swift: adds `Identifiable` protocol to structs when there's an `id` field
- swift: made all types fully `Codable`
- rust: added `@sqlxType` annotation

## [0.1.0]
- basic features, pretty much working

