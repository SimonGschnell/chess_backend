-- Add up migration script here
CREATE TABLE IF NOT EXISTS piece_colors (color TEXT NOT NULL PRIMARY KEY);
        INSERT INTO piece_colors VALUES ('BLACK');
        INSERT INTO piece_colors VALUES ('WHITE');