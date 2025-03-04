_default:
  @just --choose

stash-cli:
  @cargo build --release && cp ./target/release/fen ~/.local/bin/fen

test:
  @cargo test && cd ./integration-tests/client/SwiftClient/ && swift test && cd ../../server/rust_server && cargo test
