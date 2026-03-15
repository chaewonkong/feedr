CREATE TABLE feed (
    id          SERIAL PRIMARY KEY,
    url         TEXT NOT NULL UNIQUE,
    title       TEXT NOT NULL DEFAULT '',
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE feed_item (
    id          SERIAL PRIMARY KEY,
    feed_id     INTEGER NOT NULL REFERENCES feed(id),
    guid        TEXT NOT NULL,
    title       TEXT NOT NULL DEFAULT '',
    link        TEXT NOT NULL DEFAULT '',
    summary     TEXT NOT NULL DEFAULT '',
    published   TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(feed_id, guid)
);
