-- Your SQL goes here
CREATE TABLE follows(
    id varchar(24) PRIMARY KEY,
    source varchar(23) NOT NULL REFERENCES users,
    target varchar(23) NOT NULL REFERENCES users,
    created_on timestamp NOT NULL
)