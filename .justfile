default:
  @just --choose

help:
  @just --list

build:
  cargo build

check:
  cd db/ && just check
  cargo fmt --check

db COMMAND:
  cd db/ && just {{COMMAND}}

docs:
  cargo doc --open

deploy:
  @echo "Unsupported command."

fix:
  cd db/ && just fix
  cargo fmt

initialize:
  cd db/ && just initialize

# Generate Rust types from the Slack OpenAPI v2 endpoint.
# Note: `nix shell nixpkgs#wget nixpkgs#jdk` will provide required dependencies.
generate-slack-types:
  mkdir -p ./build
  cd build && ../scripts/generate-slack-types.sh

run:
  cargo run

test:
  cd db/ && just test
  cargo test

watch:
  cargo watch -x build

watch-fast:
  cargo watch

watch-test:
  cargo watch -x test