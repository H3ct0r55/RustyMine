CREATE TABLE IF NOT EXISTS users (
    id              INTEGER PRIMARY KEY,
    username        TEXT    NOT NULL UNIQUE,
    password_hash   TEXT    NOT NULL,
    role            TEXT    NOT NULL,
    is_active       INTEGER NOT NULL DEFAULT 1,
    email           TEXT    UNIQUE,
    created_at      TEXT    NOT NULL,
    updated_at      TEXT    NOT NULL,
    last_login_at   TEXT
);

CREATE TABLE IF NOT EXISTS servers (
    id               INTEGER PRIMARY KEY,
    name             TEXT    NOT NULL,
    slug             TEXT    NOT NULL UNIQUE,
    mc_version       TEXT    NOT NULL,
    port             INTEGER NOT NULL,
    rcon_enabled     INTEGER NOT NULL,
    rcon_port        INTEGER,
    j_max_memory_mb  INTEGER NOT NULL,
    j_min_memory_mb  INTEGER NOT NULL,
    created_at       TEXT    NOT NULL,
    updated_at       TEXT    NOT NULL,
    last_started_at  TEXT
);
