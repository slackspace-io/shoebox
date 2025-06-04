-- Add original_file_path column to videos table
-- Up migration

ALTER TABLE videos ADD COLUMN original_file_path VARCHAR(255);

-- Down migration
-- ALTER TABLE videos DROP COLUMN original_file_path;
