-- Your SQL goes here
CREATE TABLE revisions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- The page the revision is for
    page_id INTEGER NOT NULL,
    -- The user that authored this revision
    author_id INTEGER NOT NULL,
    -- Diff of the chages in this revision
    diff TEXT NOT NULL
);