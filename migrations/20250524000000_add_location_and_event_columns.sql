-- Add location and event columns to videos table
-- Up migration

ALTER TABLE videos ADD COLUMN location VARCHAR(255);
ALTER TABLE videos ADD COLUMN event VARCHAR(255);

-- Down migration
-- ALTER TABLE videos DROP COLUMN location;
-- ALTER TABLE videos DROP COLUMN event;
