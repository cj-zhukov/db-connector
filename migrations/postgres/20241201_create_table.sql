CREATE TABLE IF NOT EXISTS users (
    id TEXT NOT NULL,
    name TEXT NOT NULL
);

INSERT INTO users(id, name) VALUES
    ('a', 'foo'),
    ('b', 'bar'),
    ('c', 'baz');
