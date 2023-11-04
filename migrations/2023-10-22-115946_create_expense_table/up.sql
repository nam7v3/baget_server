-- Your SQL goes here
CREATE TABLE IF NOT EXISTS expense_table (
       expense_id INT GENERATED ALWAYS AS IDENTITY,
       user_id UUID NOT NULL,
       entry_date TEXT NOT NULL,
       amount REAL NOT NULL,
       expense TEXT NOT NULL,
       PRIMARY KEY(expense_id),
       CONSTRAINT fk_users
                  FOREIGN KEY(user_id)
                  REFERENCES users(user_id))
