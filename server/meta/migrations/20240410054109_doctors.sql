-- Add migration script here

CREATE TABLE IF NOT EXISTS doctors
(
    id VARCHAR PRIMARY KEY,
    doctor_name VARCHAR,
    speciality VARCHAR,
    experience  VARCHAR,
    hospital_address VARCHAR,
    city VARCHAR,
    pincode VARCHAR,
    is_deleted BOOLEAN NOT NULL
);



