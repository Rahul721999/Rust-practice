CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    checked BOOLEAN NOT NULL DEFAULT false
);
