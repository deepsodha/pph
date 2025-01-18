-- Add migration script here

CREATE TABLE IF NOT EXISTS users
(
    id VARCHAR PRIMARY KEY,
    mobile_number VARCHAR,
    password VARCHAR,
    role  VARCHAR,
    is_deleted BOOLEAN NOT NULL
);



