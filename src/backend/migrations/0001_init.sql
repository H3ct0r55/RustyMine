CREATE TABLE system_state (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    initialized BOOLEAN NOT NULL DEFAULT false
);

INSERT INTO system_state (id, initialized) VALUES (1, true)
ON CONFLICT (id) DO NOTHING;
