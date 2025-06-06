-- Add shoebox functionality
-- Up migration

-- Shoeboxes table
CREATE TABLE IF NOT EXISTS shoeboxes (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Video-Shoebox relationship table
CREATE TABLE IF NOT EXISTS video_shoeboxes (
    video_id VARCHAR(36) NOT NULL,
    shoebox_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (video_id, shoebox_id),
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    FOREIGN KEY (shoebox_id) REFERENCES shoeboxes (id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_shoeboxes_name ON shoeboxes (name);

-- Down migration
-- DROP TABLE IF EXISTS video_shoeboxes;
-- DROP TABLE IF EXISTS shoeboxes;
