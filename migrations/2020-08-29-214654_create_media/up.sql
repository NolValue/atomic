-- Your SQL goes here
CREATE TABLE media
(
    id varchar(25) PRIMARY KEY,
    post varchar(21),
    --Determines content type. Assumes 0 for Image, 1 for Poll, 2 for Video
    content_type smallint,
    -- The actual content. Should not mix data types, IE no Images in a Poll, or no Poll for a video.
    content JSON
)