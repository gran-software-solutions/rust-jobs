-- Add migration script here

CREATE TABLE
    users(
        user_id UUID PRIMARY KEY,
        email TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL
    );