run:
  git add .
  MATHING_CONFIG=$PWD/.config/mathing nix run

test:
  git add .
  nix build
  nix flake check --impure

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
  MATHING_CONFIG=$PWD/.config/mathing ./result/bin/seed
