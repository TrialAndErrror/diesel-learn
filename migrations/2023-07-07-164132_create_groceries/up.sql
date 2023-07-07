-- Your SQL goes here
CREATE TABLE grocery
(
    id               SERIAL PRIMARY KEY,
    amount           TEXT                        NOT NULL,
    name             TEXT                        NOT NULL,
    done             BOOLEAN                     NOT NULL DEFAULT FALSE,
    timestamp        TIMESTAMP(6) WITH TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc')
)
