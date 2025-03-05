# Changelog

## [0.5.3]
- swift: handled fractional seconds in iso8601 date decoding

## [0.5.2]
- swift: encode `nil` values with explicit `null` in JSON

## [0.5.1]
- swift: fixed naming inconsistency (`data` -> `value`)

## [0.5.0]
- tag values in enums with associated values adjacently as `data` (https://serde.rs/enum-representations.html#adjacently-tagged)
- rust: derive `Eq` and `PartialEq` for generated structs and enums
- swift: made `Response` conform to `Decodable`
- swift: add static methods `encodeAsData`, `encodeAsString`, and `decode` to `APIClient` for custom encoding/decoding
- swift: decode and encode enums adjacently

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

