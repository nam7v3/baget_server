-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
        user_id UUID DEFAULT uuid_generate_v4(),
        username VARCHAR(255) NOT NULL,
        password VARCHAR NOT NULL,
        verified BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW() NOT NULL,
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW() NOT NULL,
        PRIMARY KEY (user_id)
    );
