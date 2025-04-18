-- Your SQL goes here
CREATE TABLE media_people (
                              media_id INT REFERENCES media(id) ON DELETE CASCADE,
                              person_id INT REFERENCES people(id) ON DELETE CASCADE,
                              PRIMARY KEY (media_id, person_id)
);
