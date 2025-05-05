run:
  cargo run

test:
  cargo check
  cargo test

lint:
  test -z $(cargo fmt)
  cargo clippy -- -Dwarnings
