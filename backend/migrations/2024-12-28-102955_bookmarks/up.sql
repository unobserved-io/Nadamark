CREATE TABLE IF NOT EXISTS bookmarks (
    id INTEGER PRIMARY KEY,
    name TEXT,
    url TEXT,
    favicon_url TEXT,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    folder_id INTEGER INTEGER REFERENCES folders (id)
)
