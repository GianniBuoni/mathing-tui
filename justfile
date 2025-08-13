run:
  nix run .

test:
  cargo check
  cargo test

lint:
  test -z $(cargo fmt)
  cargo clippy --all-targets -- -Dwarnings

reset:
  rm -r .config
  just init

init:
  mkdir -p .config/mathing
  touch .config/mathing/data.db
  sqlx migrate run

seed:
  cargo run --bin seed
