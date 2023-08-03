-- Add migration script here
CREATE TABLE piece_names (name TEXT NOT NULL PRIMARY KEY);
        INSERT INTO piece_names VALUES ('PAWN'),('ROOK'),('KNIGHT'),('BISHOP'),('QUEEN'),('KING');
