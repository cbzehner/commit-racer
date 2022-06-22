default:
  @just --choose

help:
  @just --list

build:
  @just commit-racer build

check:
  @just commit-racer check
  @just db check
  @echo "Checking scripts..." && ls scripts/* | xargs shellcheck --enable all --severity style --check-sourced

commit-racer COMMAND:
  @cd commit-racer/ && just {{COMMAND}}

db COMMAND:
  @cd db/ && just {{COMMAND}}

deploy:
  @echo "Unsupported command."

fix:
  @just commit-racer fix
  @just db fix

initialize:
  # @just commit-racer fix
  @just db fix

# Generate Rust types from the Slack OpenAPI v2 endpoint.
# Note: `nix shell nixpkgs#wget nixpkgs#jdk` will provide required dependencies.
generate-slack-types:
  @mkdir -p ./build
  cd build && ../scripts/generate-slack-types.sh

run:
  @just commit-racer run

test:
  @just commit-racer test
  @just db test
