-- Add up migration script here
CREATE TABLE IF NOT EXISTS piece_names (name TEXT NOT NULL PRIMARY KEY);
        INSERT INTO piece_names VALUES ('PAWN'),('ROOK'),('KNIGHT'),('BISHOP'),('QUEEN'),('KING'),('TILE');
