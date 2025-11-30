CREATE TABLE user_permissions (
  uuid UUID PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR UNIQUE,
  password_hash VARCHAR NOT NULL,
  first_name VARCHAR,
  last_name VARCHAR
);
