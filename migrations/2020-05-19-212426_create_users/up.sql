CREATE TABLE users
(
    id character varying(23) NOT NULL PRIMARY KEY,
    url character varying(30) UNIQUE,
    nickname character varying(32),
    first_name character varying(32),
    last_name character varying(32),
    email character varying(191) UNIQUE NOT NULL,
    password character varying(191) NOT NULL
)