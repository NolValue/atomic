CREATE TABLE auths (
    id character varying (32) PRIMARY KEY,
    uid character varying (23) REFERENCES users,
    refresh_token character varying,
    access_token character varying,
    auth_expiry timestamp,
    scopes int
);