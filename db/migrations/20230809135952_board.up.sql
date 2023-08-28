-- Add up migration script here
-- Add up migration script here
CREATE TABLE IF NOT EXISTS  board (
        col TEXT NOT NULL,
        row INTEGER NOT NULL,
        field_color TEXT NOT NULL,
        has_piece INTEGER NOT NULL,
        piece_color TEXT NULL,
        piece_name TEXT NULL,
        FOREIGN KEY (piece_color,piece_name) references pieces(color,name),
        FOREIGN KEY (col) references cols(col),
        FOREIGN KEY (row) references rows(row),
        FOREIGN KEY (field_color) references piece_colors(color),
        CONSTRAINT board_PK PRIMARY KEY (col,row),
        CONSTRAINT board_unique UNIQUE (col,row),
        CONSTRAINT has_piece_booloean CHECK (has_piece=1 OR has_piece=0),
        CONSTRAINT has_piece_check CHECK (has_piece=0 AND piece_color=NULL AND piece_name=NULL OR has_piece=1 AND piece_color!=NULL AND piece_name!=NULL)
    );

INSERT INTO board VALUES ("a",1,"BLACK",1,"WHITE","ROOK");
INSERT INTO board VALUES ("b",1,"WHITE",1,"WHITE","KNIGHT");
INSERT INTO board VALUES ("c",1,"BLACK",1,"WHITE","BISHOP");
INSERT INTO board VALUES ("d",1,"WHITE",1,"WHITE","QUEEN");
INSERT INTO board VALUES ("e",1,"BLACK",1,"WHITE","KING");
INSERT INTO board VALUES ("f",1,"WHITE",1,"WHITE","BISHOP");
INSERT INTO board VALUES ("g",1,"BLACK",1,"WHITE","KNIGHT");
INSERT INTO board VALUES ("h",1,"WHITE",1,"WHITE","ROOK");

INSERT INTO board VALUES ("a",2,"WHITE",1,"WHITE","PAWN");
INSERT INTO board VALUES ("b",2,"BLACK",1,"WHITE","PAWN");
INSERT INTO board VALUES ("c",2,"WHITE",1,"WHITE","PAWN");
INSERT INTO board VALUES ("d",2,"BLACK",1,"WHITE","PAWN");
INSERT INTO board VALUES ("e",2,"WHITE",1,"WHITE","PAWN");
INSERT INTO board VALUES ("f",2,"BLACK",1,"WHITE","PAWN");
INSERT INTO board VALUES ("g",2,"WHITE",1,"WHITE","PAWN");
INSERT INTO board VALUES ("h",2,"BLACK",1,"WHITE","PAWN");

INSERT INTO board VALUES ("a",3,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("b",3,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("c",3,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("d",3,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("e",3,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("f",3,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("g",3,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("h",3,"WHITE",0,"WHITE","TILE");

INSERT INTO board VALUES ("a",4,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("b",4,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("c",4,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("d",4,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("e",4,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("f",4,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("g",4,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("h",4,"BLACK",0,"BLACK","TILE");

INSERT INTO board VALUES ("a",5,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("b",5,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("c",5,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("d",5,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("e",5,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("f",5,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("g",5,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("h",5,"WHITE",0,"WHITE","TILE");

INSERT INTO board VALUES ("a",6,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("b",6,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("c",6,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("d",6,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("e",6,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("f",6,"BLACK",0,"BLACK","TILE");
INSERT INTO board VALUES ("g",6,"WHITE",0,"WHITE","TILE");
INSERT INTO board VALUES ("h",6,"BLACK",0,"BLACK","TILE");

INSERT INTO board VALUES ("a",7,"BLACK",1,"BLACK","PAWN");
INSERT INTO board VALUES ("b",7,"WHITE",1,"BLACK","PAWN");
INSERT INTO board VALUES ("c",7,"BLACK",1,"BLACK","PAWN");
INSERT INTO board VALUES ("d",7,"WHITE",1,"BLACK","PAWN");
INSERT INTO board VALUES ("e",7,"BLACK",1,"BLACK","PAWN");
INSERT INTO board VALUES ("f",7,"WHITE",1,"BLACK","PAWN");
INSERT INTO board VALUES ("g",7,"BLACK",1,"BLACK","PAWN");
INSERT INTO board VALUES ("h",7,"WHITE",1,"BLACK","PAWN");

INSERT INTO board VALUES ("a",8,"WHITE",1,"BLACK","ROOK");
INSERT INTO board VALUES ("b",8,"BLACK",1,"BLACK","KNIGHT");
INSERT INTO board VALUES ("c",8,"WHITE",1,"BLACK","BISHOP");
INSERT INTO board VALUES ("d",8,"BLACK",1,"BLACK","QUEEN");
INSERT INTO board VALUES ("e",8,"WHITE",1,"BLACK","KING");
INSERT INTO board VALUES ("f",8,"BLACK",1,"BLACK","BISHOP");
INSERT INTO board VALUES ("g",8,"WHITE",1,"BLACK","KNIGHT");
INSERT INTO board VALUES ("h",8,"BLACK",1,"BLACK","ROOK");
    