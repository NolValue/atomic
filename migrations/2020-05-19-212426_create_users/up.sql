CREATE TABLE users
(
    id character varying(23) NOT NULL PRIMARY KEY,
    url character varying(30),
    nickname character varying(32) NOT NULL,
    first_name character varying(32),
    last_name character varying(32),
    email character varying(191),
    password character varying(191)
)