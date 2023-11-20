-- Your SQL goes here
CREATE TABLE IF NOT EXISTS account_table (
       account_id INT NOT NULL,
       user_id UUID NOT NULL,
       account TEXT NOT NULL,
       balance REAL NOT NULL,
       income REAL NOT NULL,
       expense REAL NOT NULL,
       PRIMARY KEY(account_id, user_id),
       CONSTRAINT fk_users
                  FOREIGN KEY(user_id)
                  REFERENCES users(user_id))
