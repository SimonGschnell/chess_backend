-- Add migration script here
CREATE TABLE rows(
        row INTEGER NOT NULL PRIMARY KEY,
        CONSTRAINT row_range CHECK (row>0 AND row<9)
    );
    
    INSERT INTO rows VALUES 
    (1),
    (2),
    (3),
    (4),
    (5),
    (6),
    (7),
    (8);

    CREATE TABLE cols (
        col TEXT PRIMARY KEY NOT NULL,
        CONSTRAINT row_range CHECK (col>='a' AND col<='h')
    );

    INSERT INTO cols VALUES 
    ('a'),
    ('b'),
    ('c'),
    ('d'),
    ('e'),
    ('f'),
    ('g'),
    ('h');