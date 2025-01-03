_default:
  @just --choose

stash-cli:
  @cargo build --release && cp ./target/release/cli ~/.local/bin/fen
