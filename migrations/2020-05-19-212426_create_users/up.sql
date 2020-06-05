CREATE TABLE users
(
    id character varying(23) COLLATE pg_catalog."default" NOT NULL,
    url character varying(30) COLLATE pg_catalog."default",
    username character varying(32) COLLATE pg_catalog."default" NOT NULL,
    first_name character varying(32) COLLATE pg_catalog."default",
    last_name character varying(32) COLLATE pg_catalog."default",
    email character varying(191) COLLATE pg_catalog."default",
    password character varying(191) COLLATE pg_catalog."default",
)