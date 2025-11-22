DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS servers;

CREATE TABLE IF NOT EXISTS servers (
  id                INTEGER PRIMARY KEY,
  name              TEXT NOT NULL,
  slug              TEXT NOT NULL UNIQUE,
  is_active         INTEGER NOT NULL,
  path              TEXT NOT NULL,
  jar_path          TEXT NOT NULL,
  j_max_mem         TEXT NOT NULL,
  j_min_mem         TEXT NOT NULL,
  mc_type           TEXT NOT NULL,
  mc_version        TEXT NOT NULL,
  created_at        TEXT NOT NULL,
  updated_at        TEXT NOT NULL
);


