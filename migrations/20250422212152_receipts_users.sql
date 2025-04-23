-- Up
CREATE TABLE receipts_users (
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  receipt_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  FOREIGN KEY(receipt_id) REFERENCES receipts(id)
    ON DELETE CASCADE,
  FOREIGN KEY(user_id) REFERENCES users(id)
    ON DELETE CASCADE,
  UNIQUE(receipt_id, user_id)
);
