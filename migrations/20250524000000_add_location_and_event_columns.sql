-- Add location and event columns to videos table
-- Up migration

ALTER TABLE videos ADD COLUMN location TEXT;
ALTER TABLE videos ADD COLUMN event TEXT;

-- Down migration
-- ALTER TABLE videos DROP COLUMN location;
-- ALTER TABLE videos DROP COLUMN event;
