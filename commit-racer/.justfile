build:
  cargo build

check:
  cargo fmt --check
  @just prepare-check

docs:
  cargo doc --open

fix:
  cargo fmt
  @just prepare

prepare:
  cargo sqlx prepare --database-url postgres://commit_racer:@localhost:5432/commit_racer -- --lib

prepare-check:
  cargo sqlx prepare --check --database-url postgres://commit_racer:@localhost:5432/commit_racer -- --lib

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
