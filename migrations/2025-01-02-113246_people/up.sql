-- Your SQL goes here
CREATE TABLE people (
                        id SERIAL PRIMARY KEY,
                        name TEXT UNIQUE NOT NULL
);
