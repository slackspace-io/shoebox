-- Add exif_data column to videos table
-- Up migration
ALTER TABLE videos ADD COLUMN exif_data JSONB;

-- Down migration
-- ALTER TABLE videos DROP COLUMN exif_data;
