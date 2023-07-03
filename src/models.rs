use std::{
    borrow::BorrowMut,
    cell::{Cell, Ref, RefCell},
    error::Error,
    fmt::Display,
    ops::Add,
    str::FromStr,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    file: char,
    rank: u8,
}

impl Position {
    pub fn new(file: char, rank: u8) -> Self {
        Position { file, rank }
    }

    pub fn new_from_index(row: usize, col: usize) -> Self {
        let file = match col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',

            _ => 'x',
        };
        Position {
            file,
            rank: (row + 1) as u8,
        }
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
type Matrix = Vec<Vec<RefCell<Tile>>>;
#[derive(Clone)]
pub struct Board {
    board: Arc<Mutex<Matrix>>,
}

impl Board {
    pub fn move_piece(&self, start: &Position, end: &Position) -> Result<(), String> {
        let lock = self.board.lock().unwrap();
        let moves = self
            .get_tile(start, &lock)
            .borrow_mut()
            .piece
            .as_mut()
            .expect("No piece found in start position")
            .get_moves(start, self, &lock);

        if moves.contains(end) {
            let mut tile = self.get_tile(end, &lock).borrow_mut();
            let mut piece = self
                .get_tile(start, &lock)
                .borrow_mut()
                .piece
                .take()
                .unwrap();

            if let GameObject::Pawn(pawn) = &mut piece {
                pawn.did_move = true;
            }

            tile.add_piece(piece);
            Ok(())
        } else {
            Err(String::from("illegal move, piece cant move there"))
        }
    }

    fn is_piece_in_position<'a>(
        &self,
        pos: &Position,
        lock: &'a MutexGuard<Matrix>,
    ) -> Option<Color> {
        let tile = self.get_tile(pos, &lock);
        let mut res = None;
        if let Some(piece) = tile.borrow().piece.as_ref() {
            res = Some(piece.get_color());
        } else {
            res = None;
        }
        res
    }

    fn is_piece_in_position_of_same_color<'a>(
        &self,
        pos: &Position,
        color: &Color,
        lock: &'a MutexGuard<Matrix>,
    ) -> bool {
        match self.is_piece_in_position(pos, lock) {
            Some(piece_color) => piece_color == color.clone(),
            None => false,
        }
    }

    pub fn show_moves_of_tile(&self, pos: &Position) -> Vec<Position> {
        let (rank, file) = convert_position_to_index(&pos);
        let lock = self.board.lock().unwrap();

        let mut tile = lock.get(rank).unwrap().get(file).unwrap().borrow_mut();
        match tile.piece.borrow_mut() {
            Some(piece) => piece.get_moves(&pos, self, &lock),
            None => Vec::with_capacity(0),
        }
    }

    pub fn print_with_marked(&self, pos: &Position) {
        println!("{}-{}", pos.file, pos.rank);

        let marked = self.show_moves_of_tile(pos);
        let lock = self.board.lock().unwrap();
        println!("  A B C D E F G H ");
        for i in 0..=5 {
            print!("{} ", i + 1);
            for j in 0..=7 {
                let pos = Position::new_from_index(i, j);
                if marked.contains(&pos) {
                    if self.is_piece_in_position(&pos, &lock).is_some() {
                        print!("🞩 ");
                    } else {
                        print!("🟢");
                    }
                } else {
                    print!("{}", self.get_tile(&pos, &lock).borrow().symbol());
                }
            }
            println!("");
        }
    }

    fn get_tile<'a>(&self, pos: &Position, lock: &'a MutexGuard<Matrix>) -> &'a RefCell<Tile> {
        let (rank, file) = convert_position_to_index(pos);

        lock.get(rank).unwrap().get(file).unwrap()
    }
}

fn convert_position_to_index(pos: &Position) -> (usize, usize) {
    let rank = match pos.rank {
        num @ 1..=8 => (num - 1) as usize,

        _ => panic!("rank was not allowed"),
    };
    let file = match pos.file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("file was not allowed"),
    };
    (rank, file)
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
            let r = row
                .iter()
                .map(|item| item.borrow_mut().symbol())
                .collect::<Vec<&str>>();
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
    let mut black_start: Vec<RefCell<Tile>> = Vec::with_capacity(8);
    let mut white_start: Vec<RefCell<Tile>> = Vec::with_capacity(8);

    for i in 0..8 {
        if i % 2 == 0 {
            black_start.push(Tile::new(None, Color::Black));
            white_start.push(Tile::new(None, Color::White));
        } else {
            black_start.push(Tile::new(None, Color::White));
            white_start.push(Tile::new(None, Color::Black));
        }
    }
    let black_pawns = white_start
        .clone()
        .into_iter()
        .map(|tile| {
            tile.borrow_mut()
                .add_piece(GameObject::Pawn(Pawn::new(Color::Black)));
            tile
        })
        .collect();
    let mut white_pawns: Vec<RefCell<Tile>> = black_start
        .clone()
        .into_iter()
        .map(|tile| {
            tile.borrow_mut()
                .add_piece(GameObject::Rook(Rook::new(Color::White)));
            tile
        })
        .collect();
    white_pawns[0] = Tile::new(
        Some(GameObject::Bishop(Bishop::new(Color::White))),
        Color::Black,
    );
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
    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position>;
    fn get_color(&self) -> Color;
}

//fn symbol(&self) -> &'static str;
#[derive(Clone, Debug)]
enum GameObject {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
}

impl Piece for GameObject {
    fn symbol(&self) -> &'static str {
        match self {
            GameObject::Pawn(val) => val.symbol(),
            GameObject::Rook(val) => val.symbol(),
            GameObject::Knight(val) => val.symbol(),
            GameObject::Bishop(val) => val.symbol(),
        }
    }
    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position> {
        match self {
            GameObject::Pawn(val) => val.get_moves(pos, db, lock),
            GameObject::Rook(val) => val.get_moves(pos, db, lock),
            GameObject::Knight(val) => val.get_moves(pos, db, lock),
            GameObject::Bishop(val) => val.get_moves(pos, db, lock),
        }
    }
    fn get_color(&self) -> Color {
        match self {
            GameObject::Pawn(val) => val.get_color(),
            GameObject::Rook(val) => val.get_color(),
            GameObject::Knight(val) => val.get_color(),
            GameObject::Bishop(val) => val.get_color(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
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
    fn new(piece: Option<GameObject>, color: Color) -> RefCell<Self> {
        RefCell::new(Tile { piece, color })
    }
    fn add_piece(&mut self, piece: GameObject) {
        self.piece = Some(piece);
    }
}

//? implementation of pieces
mod pieces;
use pieces::{Bishop, Knight, Pawn, Rook};

/*




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
