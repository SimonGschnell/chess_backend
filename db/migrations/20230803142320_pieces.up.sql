-- Add up migration script here
CREATE TABLE IF NOT EXISTS pieces (
            color TEXT NOT NULL ,
            name TEXT NOT NULL ,
            symbol TEXT NOT NULL,
            range INTEGER NOT NULL,
            FOREIGN KEY (color) REFERENCES piece_colors(color),
            FOREIGN KEY (name) REFERENCES piece_names(name),
            Constraint Pieces_PK Primary Key (color, name)
            Constraint Unique_PK UNIQUE (color, name),
            CONSTRAINT color_match CHECK 
            (
                symbol='♙' AND color='BLACK' AND name='PAWN' OR
                symbol='♟' AND color='WHITE' AND name='PAWN' OR
                symbol='♖' AND color='BLACK' AND name='ROOK' OR
                symbol='♜' AND color='WHITE' AND name='ROOK' OR
                symbol='♘' AND color='BLACK' AND name='KNIGHT' OR
                symbol='♞' AND color='WHITE' AND name='KNIGHT' OR
                symbol='♗' AND color='BLACK' AND name='BISHOP' OR
                symbol='♝' AND color='WHITE' AND name='BISHOP' OR
                symbol='♕' AND color='BLACK' AND name='QUEEN' OR
                symbol='♛' AND color='WHITE' AND name='QUEEN' OR
                symbol='♔' AND color='BLACK' AND name='KING' OR
                symbol='♚' AND color='WHITE' AND name='KING' 
            )
            );

        INSERT INTO pieces VALUES 
        ('BLACK', 'PAWN','♙',2),
        ('WHITE', 'PAWN','♟',2),   
        ('BLACK', 'ROOK','♖',8),  
        ('WHITE', 'ROOK','♜',8),
        ('BLACK', 'KNIGHT','♘',2),  
        ('WHITE', 'KNIGHT','♞',2),
        ('BLACK', 'BISHOP','♗',8),  
        ('WHITE', 'BISHOP','♝',8),
        ('BLACK', 'QUEEN','♕',8),  
        ('WHITE', 'QUEEN','♛',8),
        ('BLACK', 'KING','♔',1),  
        ('WHITE', 'KING','♚',1);