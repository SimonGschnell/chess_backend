use std::{
    error::Error,
    fmt::Display,
    ops::Add,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct Position {
    file: char,
    rank: u8,
}

impl Position {
    pub fn new(file: char, rank: u8) -> Self {
        Position { file, rank }
    }
}

impl FromStr for Position {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 2 {
            let s = s.to_lowercase();
            let mut it = s.chars().map(|c| c as u32);
            let f = it.next().unwrap();
            let r = it.next().unwrap();
            if let file @ 97..=104 = f {
                if let rank @ 49..=56 = r {
                    let file = char::from_u32(file).unwrap();
                    let rank = char::from_u32(rank).unwrap().to_digit(10).unwrap() as u8;
                    return Ok(Position { file, rank });
                }
            }
        }

        Err(())
    }
}

#[derive(Clone)]
pub struct Board {
    board: Arc<Mutex<Vec<Vec<Tile>>>>,
}
impl Board {
    pub fn move_piece(&mut self, start: &Position, end: &Position) -> Option<()> {
        let piece = self
            .take_piece_from_position(start)
            .expect("no piece was found in tile");
        self.add_piece_to_tile(end, piece.clone());
        Some(())
    }
    fn add_piece_to_tile(&mut self, pos: &Position, piece: GameObject) -> Option<GameObject> {
        self.board
            .lock()
            .unwrap()
            .get_mut(convert_rank_to_index(pos.rank))?
            .get_mut(convert_file_to_index(pos.file))?
            .add_piece(piece.clone());
        Some(piece)
    }
    fn take_piece_from_position(&mut self, pos: &Position) -> Option<GameObject> {
        let piece = self
            .board
            .lock()
            .unwrap()
            .get_mut(convert_rank_to_index(pos.rank))?
            .get_mut(convert_file_to_index(pos.file))?
            .piece
            .take()?;
        Some(piece)
    }
}

fn convert_file_to_index(file: char) -> usize {
    match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("file was not allowed"),
    }
}
fn convert_rank_to_index(rank: u8) -> usize {
    match rank {
        num @ 1..=8 => (num - 1) as usize,

        _ => panic!("rank was not allowed"),
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ranks = 1..9;
        f.write_str("  ");
        for file in 'A'..='H' {
            f.write_str(&file.to_string().add(" "));
        }
        f.write_str("\n");
        for row in &*self.board.lock().unwrap() {
            let r = row.iter().map(|item| item.symbol()).collect::<Vec<&str>>();
            f.write_str(&ranks.next().unwrap().to_string().add(" "));
            for i in &r {
                f.write_str(i);
            }
            f.write_str("\n");
        }

        Ok(())
    }
}

pub fn get_pawn_movement(pos: Position, range: u8) -> Vec<Position> {
    //? pawn moves differently based on its color
    //? for the moment its adapted for white pawns going down the rank
    let mut positions = Vec::new();
    //todo CHANGE BOUND TO 8 AFTER INCLUDING OTHER PIECES
    let rank_bound_max = 6;
    let rank_bound_min = 1;

    let files = pos.file..='h';
    let mut files = files.skip(1);
    let rev_files = 'a'..=pos.file;
    let mut rev_files = rev_files.rev().skip(1);
    for i in 1..=range {
        let rank = pos.rank - i;
        if rank >= rank_bound_min {
            positions.push(Position {
                file: pos.file,
                rank,
            })
        }
    }
    let rank = pos.rank - 1;
    if rank >= rank_bound_min {
        if let Some(positive_file) = files.next() {
            //? only if piece is in this Position{}

            positions.push(Position {
                file: positive_file,
                rank: rank,
            })
        }
        if let Some(negative_file) = rev_files.next() {
            //? only if piece is in this Position{}
            positions.push(Position {
                file: negative_file,
                rank: rank,
            })
        }
    }
    positions
}

pub fn get_horse(pos: Position) -> Vec<Position> {
    let mut positions = Vec::new();
    //todo CHANGE BOUND TO 8 AFTER INCLUDING OTHER PIECES
    let rank_bound_max = 6;
    let rank_bound_min = 1;

    let files = pos.file..='h';
    let mut files = files.skip(1);
    let rev_files = 'a'..=pos.file;
    let mut rev_files = rev_files.rev().skip(1);

    let highest_rank = pos.rank + 2;
    let lowest_rank = pos.rank - 2;

    if let Some(positive_file) = files.next() {
        if highest_rank <= rank_bound_max {
            positions.push(Position {
                file: positive_file,
                rank: highest_rank,
            });
        }
        if lowest_rank >= rank_bound_min {
            positions.push(Position {
                file: positive_file,
                rank: lowest_rank,
            });
        }
    }

    if let Some(negative_file) = rev_files.next() {
        if highest_rank <= rank_bound_max {
            positions.push(Position {
                file: negative_file,
                rank: highest_rank,
            });
        }
        if lowest_rank >= rank_bound_min {
            positions.push(Position {
                file: negative_file,
                rank: lowest_rank,
            });
        }
    }
    let highest_rank = pos.rank + 1;
    let lowest_rank = pos.rank - 1;
    if let Some(highest_file) = files.next() {
        if lowest_rank >= rank_bound_min {
            positions.push(Position {
                file: highest_file,
                rank: lowest_rank,
            });
        }
        if highest_rank <= rank_bound_max {
            positions.push(Position {
                file: highest_file,
                rank: highest_rank,
            });
        }
    }

    if let Some(lowest_file) = rev_files.next() {
        if lowest_rank >= rank_bound_min {
            positions.push(Position {
                file: lowest_file,
                rank: lowest_rank,
            });
        }
        if highest_rank <= rank_bound_max {
            positions.push(Position {
                file: lowest_file,
                rank: highest_rank,
            });
        }
    }

    positions
}

pub fn get_orthogonals(pos: Position, range: u8) -> Vec<Position> {
    //*example orthogonals of b6
    //*diagonals if range =1 are
    //? b5 & a6 & 6c

    let mut positions = Vec::new();
    //todo CHANGE BOUND TO 8 AFTER INCLUDING OTHER PIECES
    let rank_bound_max = 6;
    let rank_bound_min = 1;

    let files = pos.file..='h';
    let mut files = files.skip(1);
    let rev_files = 'a'..=pos.file;
    let mut rev_files = rev_files.rev().skip(1);
    for i in 1..=range {
        //?same rank
        if let Some(positive_file) = files.next() {
            positions.push(Position {
                file: positive_file,
                rank: pos.rank,
            })
        }
        if let Some(negative_file) = rev_files.next() {
            positions.push(Position {
                file: negative_file,
                rank: pos.rank,
            })
        }
        //?same file
        let positive_rank = pos.rank + i;
        let negative_rank = pos.rank - i;
        if positive_rank <= rank_bound_max {
            positions.push(Position {
                file: pos.file,
                rank: positive_rank,
            })
        }
        if negative_rank >= rank_bound_min {
            positions.push(Position {
                file: pos.file,
                rank: negative_rank,
            })
        }
    }
    positions
}
pub fn get_diagonals(pos: Position, range: u8) -> Vec<Position> {
    //*example diagonals of b6
    //*diagonals if range =1 are
    //? a5 & c5

    let mut positions: Vec<Position> = Vec::new();
    //todo CHANGE BOUND TO 8 AFTER INCLUDING OTHER PIECES
    let rank_bound_max = 6;
    let rank_bound_min = 1;

    let files = pos.file..='h';
    let mut files = files.skip(1);
    let rev_files = 'a'..=pos.file;
    let mut rev_files = rev_files.rev().skip(1);
    for i in 1..=range {
        let positive_rank = pos.rank + i;
        let negative_rank = pos.rank - i;

        if let Some(positive_file) = files.next() {
            if negative_rank >= rank_bound_min {
                positions.push(Position {
                    file: positive_file,
                    rank: negative_rank,
                });
            }
            if positive_rank <= rank_bound_max {
                positions.push(Position {
                    file: positive_file,
                    rank: positive_rank,
                });
            }
            if let Some(negative_file) = rev_files.next() {
                if positive_rank <= rank_bound_max {
                    positions.push(Position {
                        file: negative_file,
                        rank: positive_rank,
                    });
                }
                if negative_rank >= rank_bound_min {
                    positions.push(Position {
                        file: negative_file,
                        rank: negative_rank,
                    });
                };
            }
        };
    }

    positions
}

pub fn create_game() -> Board {
    let mut rows = Vec::with_capacity(8);
    /* let mut white_row: Vec<Box<dyn Piece>> = vec![
        Box::new(WhiteRook {}),
        Box::new(WhiteKnight {}),
        Box::new(WhiteBishop {}),
        Box::new(WhiteQueen {}),
        Box::new(WhiteKing {}),
        Box::new(WhiteBishop {}),
        Box::new(WhiteKnight {}),
        Box::new(WhiteRook {}),
    ]; */
    /* let mut black_row: Vec<Box<dyn Piece>> = vec![
        Box::new(BlackRook {}),
        Box::new(BlackKnight {}),
        Box::new(BlackBishop {}),
        Box::new(BlackQueen {}),
        Box::new(BlackKing {}),
        Box::new(BlackBishop {}),
        Box::new(BlackKnight {}),
        Box::new(BlackRook {}),
    ]; */
    let mut black_start: Vec<Tile> = Vec::with_capacity(8);
    let mut white_start: Vec<Tile> = Vec::with_capacity(8);

    for i in 0..8 {
        if i % 2 == 0 {
            black_start.push(Tile::new(None, Color::Black));
            white_start.push(Tile::new(None, Color::White));
        } else {
            black_start.push(Tile::new(None, Color::White));
            white_start.push(Tile::new(None, Color::Black));
        }
    }
    let black_pawns = black_start
        .clone()
        .into_iter()
        .map(|mut tile| {
            tile.add_piece(GameObject::Pawn(Pawn::new(Color::Black)));
            tile
        })
        .collect();
    let white_pawns = white_start
        .clone()
        .into_iter()
        .map(|mut tile| {
            tile.add_piece(GameObject::Pawn(Pawn::new(Color::White)));
            tile
        })
        .collect();
    rows.push(white_pawns);
    rows.push(white_start.clone());
    rows.push(black_start.clone());
    rows.push(white_start.clone());
    rows.push(black_start.clone());
    rows.push(black_pawns);
    let rows = Arc::new(Mutex::new(rows));

    Board { board: rows }
}

trait Piece {
    fn symbol(&self) -> &'static str;
}

//fn symbol(&self) -> &'static str;
#[derive(Clone, Debug)]
enum GameObject {
    Pawn(Pawn),
}

impl Piece for GameObject {
    fn symbol(&self) -> &'static str {
        match self {
            GameObject::Pawn(val) => val.symbol(),
        }
    }
}

#[derive(Clone, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Clone)]
struct Tile {
    color: Color,
    piece: Option<GameObject>,
}

impl Tile {
    fn symbol(&self) -> &'static str {
        if let Some(val) = &self.piece {
            return val.symbol();
        }
        match self.color {
            Color::White => chess_backend::WHITE_TILE,
            Color::Black => chess_backend::BLACK_TILE,
        }
    }
    fn new(piece: Option<GameObject>, color: Color) -> Self {
        Tile { piece, color }
    }
    fn add_piece(&mut self, piece: GameObject) {
        self.piece = Some(piece); /*
                                      if self.piece.is_none() {
                                  } else {
                                      println!(
                                          "tried to put piece {} in {}",
                                          piece.symbol(),
                                          self.piece.clone().unwrap().symbol()
                                      );
                                  } */
    }
}
/* #[derive(Clone)]
struct BlackTile {
    piece: Option<GameObject>,
}
impl BlackTile {
    fn symbol(&self) -> &'static str {
        chess_backend::BLACK_TILE
    }
    fn new(piece: Option<GameObject>) -> Self {
        BlackTile { piece: piece }
    }
} */
/* #[derive(Clone, Debug)]
struct WhitePawn {
    movement: u16,
}
impl WhitePawn {
    fn new() -> Self {
        WhitePawn { movement: 1 }
    }
}
impl Piece for WhitePawn {
    fn symbol(&self) -> &'static str {
        chess_backend::WHITE_PAWN_SYMBOL
    }
} */

#[derive(Clone, Debug)]
struct Pawn {
    did_move: bool,
    movement: u16,
    color: Color,
}

impl Pawn {
    fn new(color: Color) -> Self {
        Pawn {
            did_move: false,
            movement: 2,
            color,
        }
    }
    fn mov(&mut self) {
        self.did_move = true;
        self.movement = 1;
    }
}
impl Piece for Pawn {
    fn symbol(&self) -> &'static str {
        if let Color::Black = self.color {
            return chess_backend::BLACK_PAWN_SYMBOL;
        }
        chess_backend::WHITE_PAWN_SYMBOL
    }
}
/*
struct BlackRook {}
impl Piece for BlackRook {
    fn symbol(&self) -> &'static str {
        chess_backend::BLACK_ROOK_SYMBOL
    }
}

struct WhiteRook {}
impl Piece for WhiteRook {
    fn symbol(&self) -> &'static str {
        chess_backend::WHITE_ROOK_SYMBOL
    }
}
struct BlackKnight {}
impl Piece for BlackKnight {
    fn symbol(&self) -> &'static str {
        chess_backend::BLACK_KNIGHT_SYMBOL
    }
}
struct WhiteKnight {}
impl Piece for WhiteKnight {
    fn symbol(&self) -> &'static str {
        chess_backend::WHITE_KNIGHT_SYMBOL
    }
}

struct BlackBishop {}
impl Piece for BlackBishop {
    fn symbol(&self) -> &'static str {
        chess_backend::BLACK_BISHOP_SYMBOL
    }
}
struct WhiteBishop {}
impl Piece for WhiteBishop {
    fn symbol(&self) -> &'static str {
        chess_backend::WHITE_BISHOP_SYMBOL
    }
}
struct BlackQueen {}
impl Piece for BlackQueen {
    fn symbol(&self) -> &'static str {
        chess_backend::BLACK_QUEEN_SYMBOL
    }
}
struct WhiteQueen {}
impl Piece for WhiteQueen {
    fn symbol(&self) -> &'static str {
        chess_backend::WHITE_QUEEN_SYMBOL
    }
}
struct BlackKing {}
impl Piece for BlackKing {
    fn symbol(&self) -> &'static str {
        chess_backend::BLACK_KING_SYMBOL
    }
}
struct WhiteKing {}
impl Piece for WhiteKing {
    fn symbol(&self) -> &'static str {
        chess_backend::WHITE_KING_SYMBOL
    }
}
*/
