-- Your SQL goes here
CREATE TABLE media_tags (
                            media_id INT REFERENCES media(id) ON DELETE CASCADE,
                            tag_id INT REFERENCES tags(id) ON DELETE CASCADE,
                            PRIMARY KEY (media_id, tag_id)
);
