-- Add migration script here

CREATE TABLE IF NOT EXISTS drugs
(
    id VARCHAR PRIMARY KEY,
    brand VARCHAR,
    generic VARCHAR,
    details  VARCHAR,
    category VARCHAR,
    side_effects VARCHAR,
    drugsdose_info VARCHAR,
    precautions VARCHAR,
    manufacturer_name VARCHAR,
    medicines VARCHAR,
    contra_indications VARCHAR,
    diseases VARCHAR,
    interactions VARCHAR,
    contains VARCHAR,
    is_deleted BOOLEAN NOT NULL
);



