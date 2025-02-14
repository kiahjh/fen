_default:
  @just --choose

stash-cli:
  @cargo build --release && cp ./target/release/fen ~/.local/bin/fen

test:
  @cargo test # TODO: integration tests
