CREATE TABLE IF NOT EXISTS articles (
    created_time DATETIME NOT NULL,
    modified_time DATETIME NOT NULL,
    slug TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    subline TEXT NOT NULL,
    author_slug TEXT NOT NULL
);

INSERT INTO articles (created_time, modified_time, slug, name, subline, author_slug) VALUES ('2024-01-01T13:14:00', '2024-01-01T13:14:00', 'test-article', 'Test Article', 'This is a test article', 'test-author');
