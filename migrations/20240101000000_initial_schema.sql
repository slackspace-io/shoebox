-- Initial schema for Shoebox - a digital shoebox for your videos
-- Up migration

-- Videos table
CREATE TABLE IF NOT EXISTS videos (
    id TEXT PRIMARY KEY NOT NULL,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    title TEXT,
    description TEXT,
    created_date TEXT,
    file_size INTEGER,
    thumbnail_path TEXT,
    rating INTEGER CHECK (rating BETWEEN 1 AND 5 OR rating IS NULL),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Tags table
CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- People table
CREATE TABLE IF NOT EXISTS people (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Video-Tag relationship table
CREATE TABLE IF NOT EXISTS video_tags (
    video_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (video_id, tag_id),
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
);

-- Video-People relationship table
CREATE TABLE IF NOT EXISTS video_people (
    video_id TEXT NOT NULL,
    person_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
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
