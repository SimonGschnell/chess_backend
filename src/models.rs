use serde::Serialize;
mod pieces;
use pieces::{Bishop, King, Knight, Pawn, Queen, Rook};
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashSet,
    fmt::Display,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Serialize)]
pub struct printablePiece {
    pub col: String,
    pub row: i8,
    pub symbol: String,
}

impl FromRow<'_, SqliteRow> for printablePiece {
    fn from_row(r: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            col: r.try_get("col")?,
            row: r.try_get("row")?,
            symbol: r.try_get("symbol")?,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Eq, Hash)]
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

pub type Db = Arc<Mutex<Board>>;
type Matrix = Vec<Vec<RefCell<Tile>>>;
#[derive(Clone)]
pub struct Board {
    board: Matrix,
    players_turn: Color,
}

impl Board {
    pub fn move_piece(&mut self, start: &Position, end: &Position) -> Result<(), String> {
        let mut game_over = false;
        let moves = match self.get_tile(start).borrow_mut().piece.as_mut() {
            Some(val) => val.get_moves(start, self),
            None => {
                return Err(format!("There is no piece at {:?}", start));
            }
        };

        if self
            .get_tile(start)
            .borrow_mut()
            .piece
            .as_mut()
            .unwrap()
            .get_color()
            != self.players_turn
        {
            return Err(format!("{:?} to play!", self.players_turn));
        }

        if moves.contains(end) {
            let mut tile = self.get_tile(end).borrow_mut();
            let piece = self.get_tile(start).borrow_mut().piece.take().unwrap();

            let taken_piece = tile.add_piece(piece);

            if let Some(GameObject::King(_)) = taken_piece {
                println!("CHECKMATEEEEEEEEE, {:?} WINS!", self.players_turn);
                game_over = true;
            }
        } else {
            return Err(String::from("illegal move, piece cant move there"));
        }
        self.next_turn();
        if game_over {
            self.reset_board();
        }
        Ok(())
    }

    fn reset_board(&mut self) {
        let clean_board = create_game();
        self.board = clean_board.lock().unwrap().board.clone();
        self.players_turn = Color::White;
    }

    fn next_turn(&mut self) {
        match self.players_turn {
            Color::Black => {
                self.players_turn = Color::White;
            }
            Color::White => {
                self.players_turn = Color::Black;
            }
        }
    }

    pub fn is_check(&self) -> bool {
        let possible_takes = self.get_all_possible_takes();

        for possible in possible_takes {
            if let GameObject::King(_) = self
                .get_tile(&possible)
                .borrow_mut()
                .piece
                .as_ref()
                .unwrap()
            {
                return true;
            }
        }
        false
    }

    fn get_all_possible_takes(&self) -> HashSet<Position> {
        let mut possible_takes = HashSet::new();

        for (row, i) in self
            .board
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &Vec<RefCell<Tile>>)>>()
        {
            for (col, j) in i
                .iter()
                .enumerate()
                .collect::<Vec<(usize, &RefCell<Tile>)>>()
            {
                if let Some(val) = j.borrow_mut().piece.as_mut() {
                    for pos in val.get_moves(&Position::new_from_index(row, col), self) {
                        if self.is_piece_in_position(&pos).is_some() {
                            possible_takes.insert(pos);
                        }
                    }
                }
            }
        }
        possible_takes
    }

    fn is_piece_in_position(&self, pos: &Position) -> Option<Color> {
        let tile = self.get_tile(pos);

        tile.borrow().piece.as_ref().map(|piece| piece.get_color())
    }

    fn is_piece_in_position_of_same_color(&self, pos: &Position, color: &Color) -> bool {
        match self.is_piece_in_position(pos) {
            Some(piece_color) => piece_color == color.clone(),
            None => false,
        }
    }

    pub fn show_moves_of_tile(&self, pos: &Position) -> Vec<Position> {
        let (rank, file) = convert_position_to_index(pos);

        let mut tile = self
            .board
            .get(rank)
            .unwrap()
            .get(file)
            .unwrap()
            .borrow_mut();
        match tile.piece.borrow_mut() {
            Some(piece) => piece.get_moves(pos, self),
            None => Vec::with_capacity(0),
        }
    }

    pub fn print_with_marked(&self, pos: &Position) {
        println!("{}-{}", pos.file, pos.rank);

        let marked = self.show_moves_of_tile(pos);

        println!("  A B C D E F G H ");
        for i in 0..=7 {
            print!("{} ", i + 1);
            for j in 0..=7 {
                let pos = Position::new_from_index(i, j);
                if marked.contains(&pos) {
                    if self.is_piece_in_position(&pos).is_some() {
                        print!("🞩 ");
                    } else {
                        print!("🟢");
                    }
                } else {
                    print!("{}", self.get_tile(&pos).borrow().symbol());
                }
            }
            println!();
        }
        println!("{:?} to Move!", self.players_turn);
    }

    fn get_tile(&self, pos: &Position) -> &RefCell<Tile> {
        let (rank, file) = convert_position_to_index(pos);

        self.board.get(rank).unwrap().get(file).unwrap()
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

        let board = match self.players_turn {
            Color::Black => self
                .board
                .iter()
                .enumerate()
                .collect::<Vec<(usize, &Vec<RefCell<Tile>>)>>(),
            Color::White => {
                let mut b = self
                    .board
                    .iter()
                    .enumerate()
                    .collect::<Vec<(usize, &Vec<RefCell<Tile>>)>>();
                b.reverse();
                b
            }
        };

        for i in board {
            let mut rank = (i.0 + 1).to_string();
            rank.push(' ');
            f.write_str(&rank)?;
            for j in i.1 {
                f.write_str(j.borrow().symbol())?
            }
            f.write_str("\n")?
        }
        f.write_fmt(format_args!("{:?} to Move!", self.players_turn))?;

        Ok(())
    }
}

pub fn create_game() -> Arc<Mutex<Board>> {
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
    let rows = rows;

    Arc::new(Mutex::new(Board {
        board: rows,
        players_turn: Color::White,
    }))
}

trait Piece {
    fn symbol(&self) -> &'static str;
    fn get_moves(&mut self, pos: &Position, db: &Board) -> Vec<Position>;
    fn get_color(&self) -> Color;
    fn set_color(&mut self, color: Color);
}

//fn symbol(&self) -> &'static str;
#[derive(Clone, Debug, Serialize)]
pub enum GameObject {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
}

impl FromRow<'_, SqliteRow> for GameObject {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(match row.try_get("name")? {
            "PAWN" => GameObject::Pawn(Pawn::from_row(row)?),
            "ROOK" => GameObject::Rook(Rook::from_row(row)?),
            "KNIGHT" => GameObject::Knight(Knight::from_row(row)?),
            "BISHOP" => GameObject::Bishop(Bishop::from_row(row)?),
            "QUEEN" => GameObject::Queen(Queen::from_row(row)?),
            "KING" => GameObject::King(King::from_row(row)?),
            var => panic!("type was not allowed {}", var),
        })
    }
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
    fn get_moves<'a>(&mut self, pos: &Position, db: &Board) -> Vec<Position> {
        match self {
            GameObject::Pawn(val) => val.get_moves(pos, db),
            GameObject::Rook(val) => val.get_moves(pos, db),
            GameObject::Knight(val) => val.get_moves(pos, db),
            GameObject::Bishop(val) => val.get_moves(pos, db),
            GameObject::Queen(val) => val.get_moves(pos, db),
            GameObject::King(val) => val.get_moves(pos, db),
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Serialize)]
pub struct Tile {
    color: Color,
    piece: Option<GameObject>,
}

impl FromRow<'_, SqliteRow> for Tile {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Tile {
            color: match row.try_get("field_color")? {
                "WHITE" => Color::White,
                "BLACK" => Color::Black,
                _ => panic!("not allowed field color"),
            },
            piece: match row.try_get("name")? {
                "PAWN" => Some(GameObject::Pawn(Pawn::from_row(row)?)),
                "ROOK" => Some(GameObject::Rook(Rook::from_row(row)?)),
                "KNIGHT" => Some(GameObject::Knight(Knight::from_row(row)?)),
                "BISHOP" => Some(GameObject::Bishop(Bishop::from_row(row)?)),
                "QUEEN" => Some(GameObject::Queen(Queen::from_row(row)?)),
                "KING" => Some(GameObject::King(King::from_row(row)?)),
                _ => None,
            },
        })
    }
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
    fn add_piece(&mut self, piece: GameObject) -> Option<GameObject> {
        let old = self.piece.clone();
        self.piece = Some(piece);
        old
    }
}
