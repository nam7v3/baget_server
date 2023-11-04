-- Your SQL goes here
CREATE TABLE IF NOT EXISTS transaction_table (
       transaction_id INT GENERATED ALWAYS AS IDENTITY,
       user_id UUID NOT NULL,
       entry_date TEXT NOT NULL,
       amount REAL NOT NULL,
       account TEXT NOT NULL,
       category TEXT NOT NULL,
       transaction_type TEXT NOT NULL,
       transaction_title TEXT NOT NULL,
       PRIMARY KEY(transaction_id),
       CONSTRAINT fk_users
                  FOREIGN KEY(user_id)
                  REFERENCES users(user_id))
