-- Add up migration script here

CREATE TABLE IF NOT EXISTS chess_board(
    ID INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    player_turn TEXT NOT NULL,
    board_name TEXT NOT NULL,
    FOREIGN KEY (player_turn) REFERENCES piece_colors(color),
    CONSTRAINT unique_board_name UNIQUE (board_name)
);

INSERT INTO chess_board VALUES (NULL,'WHITE','board');