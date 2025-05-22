-- Add duration column to videos table
-- Up migration

ALTER TABLE videos ADD COLUMN duration INTEGER;

-- Down migration
-- ALTER TABLE videos DROP COLUMN duration;
