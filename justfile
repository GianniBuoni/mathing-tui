run:
  nix run .

test:
  nix build .#test

lint:
  cargo fmt --check
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
