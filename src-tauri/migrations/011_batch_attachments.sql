CREATE TABLE batch_attachments (
    id            TEXT    PRIMARY KEY,
    batch_id      TEXT    NOT NULL REFERENCES batches(id) ON DELETE CASCADE,
    filename      TEXT    NOT NULL,
    original_name TEXT    NOT NULL,
    mime_type     TEXT,
    size_bytes    INTEGER NOT NULL,
    created_at    INTEGER NOT NULL
);
