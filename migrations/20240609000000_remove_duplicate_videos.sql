-- Remove duplicate videos based on file_path
-- Up migration

-- First, create a temporary table to store the IDs of duplicate videos to keep
-- We'll keep the most recently updated video for each file_path
CREATE TEMPORARY TABLE IF NOT EXISTS videos_to_keep AS
WITH ranked_videos AS (
    SELECT
        id,
        file_path,
        ROW_NUMBER() OVER (PARTITION BY file_path ORDER BY updated_at DESC) as rn
    FROM videos
)
SELECT id FROM ranked_videos WHERE rn = 1;

-- Delete videos that are not in the videos_to_keep table
DELETE FROM videos
WHERE id NOT IN (SELECT id FROM videos_to_keep);

-- Drop the temporary table
DROP TABLE IF EXISTS videos_to_keep;

-- Down migration
-- No down migration as we can't restore deleted data
