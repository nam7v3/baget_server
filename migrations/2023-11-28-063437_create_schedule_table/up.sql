-- Your SQL goes here
CREATE TABLE IF NOT EXISTS schedule_table (
       transaction_id BIGINT NOT NULL,
       user_id UUID NOT NULL,
       time_unit TEXT NOT NULL,
       time_schedule INT NOT NULL,
       last_time_added INT NOT NULL,
       PRIMARY KEY(transaction_id, user_id),
       CONSTRAINT fk_users
                  FOREIGN KEY(user_id)
                  REFERENCES users(user_id))
