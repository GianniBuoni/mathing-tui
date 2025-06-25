run:
  cargo run

test:
  cargo check
  cargo test

lint:
  test -z $(cargo fmt)
  cargo clippy -- -Dwarnings

reset:
  rm data.db
  touch data.db
  sqlx migrate run

seed:
  cargo run --bin seed
