CREATE TABLE IF NOT EXISTS folders (
    id INTEGER PRIMARY KEY,
    name TEXT,
    parent_id INTEGER REFERENCES folders (id)
)
