-- Up
CREATE TABLE items(
  id INTEGER PRIMARY KEY,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  name TEXT NOT NULL,
  price FLOAT NOT NULL
)
