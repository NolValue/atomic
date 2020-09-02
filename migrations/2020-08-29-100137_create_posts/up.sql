-- Your SQL goes here
CREATE TABLE posts
(
    id character varying(21) PRIMARY KEY,
    --Defines Source type. 0 for None, 1 for Collection, 2 for Community.
    source_type smallint default(0),
    --Collections and Communities share ID length. Simpler code this way.
    source_id character varying(22),
    public bool default (true),
    --Reshares and Comments determine whether or not interactions are enabled for post.
    reshares bool default (true),
    comments bool default (true),
    poster character varying (23) REFERENCES users NOT NULL,
    content text NOT NULL,
    created_on timestamp NOT NULL
)