-- Add up migration script here
CREATE TABLE IF NOT EXISTS board_counter(
id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT
);

INSERT INTO board_counter VALUES (NULL);