-- Your SQL goes here
CREATE TABLE media
(
    id          SERIAL PRIMARY KEY,
    root_path   TEXT        NOT NULL,
    route       TEXT        NOT NULL,
    file_name   TEXT        NOT NULL,
    file_path   TEXT        NOT NULL unique,
    media_type  TEXT        NOT NULL CHECK (media_type IN ('photo', 'video')),
    usable      BOOLEAN              DEFAULT TRUE,
    highlight   BOOLEAN              DEFAULT FALSE,
    reviewed    BOOLEAN              DEFAULT FALSE,
    description TEXT,
    duration_ms INTEGER              DEFAULT 0,
    created_at  TIMESTAMPtz NOT NULL DEFAULT NOW(),
    uploaded_at timestamptz          DEFAULT NOW()
);
