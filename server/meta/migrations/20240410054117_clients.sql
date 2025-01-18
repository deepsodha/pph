-- Add migration script here

CREATE TABLE IF NOT EXISTS clients
(
    id VARCHAR PRIMARY KEY,
    name VARCHAR,
    address VARCHAR,
    mobile_number  VARCHAR,
    email VARCHAR,
    gst_no VARCHAR,
    pan_no VARCHAR,
    is_deleted BOOLEAN NOT NULL
);



