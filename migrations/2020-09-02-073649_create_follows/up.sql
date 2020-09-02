-- Your SQL goes here
CREATE TABLE follows(
    source varchar(23) PRIMARY KEY,
    target varchar(23),
    created_on timestamp
)