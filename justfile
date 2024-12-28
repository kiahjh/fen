_default:
  @just --choose

stash-cli:
  @cd cli && cargo build --release && cp ./target/release/fen ~/.local/bin/fen

add-cli-to-packages:
  @cd cli && cargo build --release && cd .. && cp ./cli/target/release/fen ./client/ts/

build-cli:
  @just stash-cli && just add-cli-to-packages

# demos:
solid-hono-frontend:
  @cd ./demos/solid-hono/frontend && bun dev

solid-hono-backend:
  @cd ./demos/solid-hono/api && bun dev

