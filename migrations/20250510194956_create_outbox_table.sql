CREATE TABLE IF NOT EXISTS outbox (
    id UUID NOT NULL PRIMARY KEY,
    domain TEXT NOT NULL,
    action TEXT NOT NULL,
    type TEXT NOT NULL,
    key TEXT NOT NULL,
    payload BYTEA NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);
