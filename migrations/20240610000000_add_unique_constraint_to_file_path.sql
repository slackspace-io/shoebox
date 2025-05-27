-- Add unique constraint to file_path column in videos table
-- Up migration
CREATE UNIQUE INDEX IF NOT EXISTS idx_videos_file_path_unique ON videos (file_path);

-- Down migration
-- DROP INDEX IF EXISTS idx_videos_file_path_unique;
