use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fmt::Display,
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
        Position::new(file, (row + 1) as u8)
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
                    return Ok(Position::new(file, rank));
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
        let moves = match self.get_tile(start, &lock).borrow_mut().piece.as_mut() {
            Some(val) => val.get_moves(start, self, &lock),
            None => {
                return Err(format!("There is no piece at {:?}", start));
            }
        };

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

        if let Some(piece) = tile.borrow().piece.as_ref() {
            Some(piece.get_color())
        } else {
            None
        }
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
        for i in 0..=7 {
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
    //todo this function can panic

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
        f.write_str("  A B C D E F G H\n")?;
        for i in &*self
            .board
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &Vec<RefCell<Tile>>)>>()
        {
            let mut rank = i.0.to_string();
            rank.push(' ');
            f.write_str(&rank)?;
            for j in i.1 {
                f.write_str(j.borrow().symbol())?
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}
pub fn create_game() -> Board {
    let mut rows = Vec::with_capacity(8);

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
    let white_pawns: Vec<RefCell<Tile>> = black_start
        .clone()
        .into_iter()
        .map(|tile| {
            tile.borrow_mut()
                .add_piece(GameObject::Pawn(Pawn::new(Color::White)));
            tile
        })
        .collect();
    let mut pieces = [
        GameObject::Rook(Rook::new(Color::White)),
        GameObject::Knight(Knight::new(Color::White)),
        GameObject::Bishop(Bishop::new(Color::White)),
        GameObject::King(King::new(Color::White)),
        GameObject::Queen(Queen::new(Color::White)),
        GameObject::Bishop(Bishop::new(Color::White)),
        GameObject::Knight(Knight::new(Color::White)),
        GameObject::Rook(Rook::new(Color::White)),
    ];
    let white_pieces: Vec<RefCell<Tile>> = white_start
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, tile)| {
            tile.borrow_mut().add_piece(pieces[i].to_owned());
            tile
        })
        .collect();

    let black_pieces: Vec<RefCell<Tile>> = black_start
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, tile)| {
            pieces[i].set_color(Color::Black);
            tile.borrow_mut().add_piece(pieces[i].to_owned());
            tile
        })
        .collect();

    rows.push(white_pieces);
    rows.push(white_pawns);
    rows.push(white_start.clone());
    rows.push(black_start.clone());
    rows.push(white_start.clone());
    rows.push(black_start.clone());
    rows.push(black_pawns);
    rows.push(black_pieces);
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
    fn set_color(&mut self, color: Color);
}

//fn symbol(&self) -> &'static str;
#[derive(Clone, Debug)]
enum GameObject {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
}

impl Piece for GameObject {
    fn symbol(&self) -> &'static str {
        match self {
            GameObject::Pawn(val) => val.symbol(),
            GameObject::Rook(val) => val.symbol(),
            GameObject::Knight(val) => val.symbol(),
            GameObject::Bishop(val) => val.symbol(),
            GameObject::Queen(val) => val.symbol(),
            GameObject::King(val) => val.symbol(),
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
            GameObject::Queen(val) => val.get_moves(pos, db, lock),
            GameObject::King(val) => val.get_moves(pos, db, lock),
        }
    }
    fn get_color(&self) -> Color {
        match self {
            GameObject::Pawn(val) => val.get_color(),
            GameObject::Rook(val) => val.get_color(),
            GameObject::Knight(val) => val.get_color(),
            GameObject::Bishop(val) => val.get_color(),
            GameObject::Queen(val) => val.get_color(),
            GameObject::King(val) => val.get_color(),
        }
    }
    fn set_color(&mut self, color: Color) {
        match self {
            GameObject::Pawn(val) => val.set_color(color),
            GameObject::Rook(val) => val.set_color(color),
            GameObject::Knight(val) => val.set_color(color),
            GameObject::Bishop(val) => val.set_color(color),
            GameObject::Queen(val) => val.set_color(color),
            GameObject::King(val) => val.set_color(color),
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
use pieces::{Bishop, King, Knight, Pawn, Queen, Rook};
