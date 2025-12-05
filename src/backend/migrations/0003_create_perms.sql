CREATE TABLE user_permissions(
  uuid UUID PRIMARY KEY,
  root BOOL NOT NULL,
  permissions JSON NOT NULL
);
