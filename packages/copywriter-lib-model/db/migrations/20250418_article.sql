CREATE TABLE IF NOT EXISTS articles (
    slug TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    subline TEXT NOT NULL,
    author_slug TEXT NOT NULL,
    published_timestamp TEXT NOT NULL
);

INSERT INTO articles (slug, name, subline, author_slug, published_timestamp) VALUES ('test-article', 'Test Article', 'This is a test article', 'test-author', '2023-01-01');
