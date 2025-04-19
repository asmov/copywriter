CREATE TABLE IF NOT EXISTS articles (
    slug TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    subline TEXT NOT NULL,
    author_slug TEXT NOT NULL,
    published_timestamp TEXT NOT NULL,
);
