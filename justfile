_default:
  @just --choose

stash-cli:
  @cargo build --release && cp ./target/release/fen ~/.local/bin/fen

test:
  @cargo test && cd ./integration-tests/client/SwiftClient/ && swift test && cd ../../server/rust_server && cargo test

test-watch:
  @watchexec -i "./**" just test

publish-parser:
  @cargo publish -p fen_parser

publish-cli:
  @cargo publish -p fen_cli
