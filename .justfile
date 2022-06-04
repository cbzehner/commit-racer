default:
  @just --choose

help:
  @just --list

build:
  cargo build

check:
  cargo fmt --check

docs:
  cargo doc --open

deploy:
  @echo "Unsupported command."

format:
  cargo fmt

initialize:
  @echo "Unimplemented!"

# Generate Rust types from the Slack OpenAPI v2 endpoint.
# Note: `nix shell nixpkgs#wget nixpkgs#jdk` will provide required dependencies.
generate-slack-types:
  mkdir -p ./build
  cd build && ../scripts/generate-slack-types.sh

run:
  cargo run

test:
  cargo test

watch:
  cargo watch -x build

watch-fast:
  cargo watch

watch-test:
  cargo watch -x test