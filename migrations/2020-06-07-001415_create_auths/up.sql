CREATE TABLE auths (
    id character varying (27) PRIMARY KEY,
    uid character varying (23) REFERENCES users NOT NULL,
    refresh_token character varying (32) UNIQUE NOT NULL,
    access_token character varying (36) UNIQUE NOT NULL,
    auth_expiry timestamp NOT NULL,
    nickname character varying (100),
    scopes int DEFAULT (1)
);