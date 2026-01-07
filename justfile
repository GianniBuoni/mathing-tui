# runs the progam via nix, uses env variables to use local config file
run:
  git add .
  MATHING_CONFIG=$PWD/.config/mathing nix run
# builds and tests program via nix
test:
  git add .
  nix build
  nix flake check --impure
# lints program using cargo
lint:
  cargo fmt --check
  cargo clippy --all-targets -- -Dwarnings
# removes the current local config and db file and runs a sqlx migration
reset:
  rm -r .config
  just init
# runs a sqlx migration
init:
  mkdir -p .config/mathing
  touch .config/mathing/data.db
  sqlx migrate run
# seeds an empty database with dummy data 
seed:
  MATHING_CONFIG=$PWD/.config/mathing ./result/bin/seed
