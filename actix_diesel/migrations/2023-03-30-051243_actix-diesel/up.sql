-- Your SQL goes here
CREATE TABLE todo_list(
  id SERIAL PRIMARY KEY,
  title VARCHAR(150)
);
CREATE TABLE todo_item (
  id SERIAL PRIMARY KEY,
  title VARCHAR(150) UNIQUE NOT NULL,
  checked BOOLEAN NOT NULL DEFAULT false
);