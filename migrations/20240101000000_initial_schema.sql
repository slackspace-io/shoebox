-- Initial schema for Shoebox - a digital shoebox for your videos
-- Up migration

-- Videos table
CREATE TABLE IF NOT EXISTS videos (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    title VARCHAR(255),
    description TEXT,
    created_date VARCHAR(50),
    file_size BIGINT,
    thumbnail_path VARCHAR(255),
    rating INTEGER CHECK (rating BETWEEN 1 AND 5 OR rating IS NULL),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Tags table
CREATE TABLE IF NOT EXISTS tags (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- People table
CREATE TABLE IF NOT EXISTS people (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Video-Tag relationship table
CREATE TABLE IF NOT EXISTS video_tags (
    video_id VARCHAR(36) NOT NULL,
    tag_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (video_id, tag_id),
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- Video-People relationship table
CREATE TABLE IF NOT EXISTS video_people (
    video_id VARCHAR(36) NOT NULL,
    person_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (video_id, person_id),
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    FOREIGN KEY (person_id) REFERENCES people (id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_videos_file_path ON videos (file_path);
CREATE INDEX IF NOT EXISTS idx_videos_created_date ON videos (created_date);
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags (name);
CREATE INDEX IF NOT EXISTS idx_people_name ON people (name);

-- Down migration
-- DROP TABLE IF EXISTS video_people;
-- DROP TABLE IF EXISTS video_tags;
-- DROP TABLE IF EXISTS people;
-- DROP TABLE IF EXISTS tags;
-- DROP TABLE IF EXISTS videos;
