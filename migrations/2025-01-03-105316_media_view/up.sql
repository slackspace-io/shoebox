-- Your SQL goes here
CREATE VIEW media_view AS
SELECT
    media.id AS media_id,
    media.file_name,
    media.file_path,
    media.media_type,
    media.reviewed,
    media.description,
    media.created_at,
    media.uploaded_at,
    tags.id AS tag_id,
    tags.name AS tag_name
FROM
    media
        LEFT JOIN media_tags ON media_tags.media_id = media.id
        LEFT JOIN tags ON tags.id = media_tags.tag_id;
