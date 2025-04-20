-- Add migration script here
CREATE TABLE IF NOT EXISTS items(
  id INTEGER PRIMARY KEY,
  price FLOAT NOT NULL,
  name TEXT NOT NULL
)
