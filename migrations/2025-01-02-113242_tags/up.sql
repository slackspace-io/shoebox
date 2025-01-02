-- Your SQL goes here
CREATE TABLE tags (
                      id SERIAL PRIMARY KEY,
                      name TEXT UNIQUE NOT NULL
);
