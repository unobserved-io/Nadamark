CREATE TABLE IF NOT EXISTS folders (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    parent_id INTEGER REFERENCES folders(id) ON DELETE CASCADE,
    favorite BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (parent_id) REFERENCES folders(id) ON DELETE CASCADE
);

CREATE INDEX idx_folders_parent_id ON folders(parent_id);
