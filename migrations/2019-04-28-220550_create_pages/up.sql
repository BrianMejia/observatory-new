-- Your SQL goes here
CREATE TABLE pages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- Title of the page
    title TEXT NOT NULL,
    -- JSON Array of tags
    tags TEXT NOT NULL,
    -- Minimum privledge tier to view
    view_tier INTEGER NOT NULL,
    -- Minimum privledge tier to edit
    edit_tier INTEGER NOT NULL
);