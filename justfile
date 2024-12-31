_default:
  @just --choose

stash-cli:
  @cd cli && cargo build --release && cp ./target/release/fen ~/.local/bin/fen
