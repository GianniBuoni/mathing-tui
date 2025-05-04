run:
  cargo check
  cargo test
  cargo run

test:
  cargo check
  cargo clippy -- -Dwarnings
  cargo test
