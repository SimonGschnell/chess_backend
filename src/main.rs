use crate::models::Board;
use unicode_width::UnicodeWidthChar;
use warp::Filter;

#[tokio::main]
async fn main() {
    const RUST_LOG: &str = "RUST_LOG";
    if std::env::var_os(RUST_LOG).is_none() {
        std::env::set_var(RUST_LOG, "chess=info");
    }
    pretty_env_logger::init();
    println!("\n  A B C D E F G H");
    println!("1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖");
    println!("2 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙");
    //println!("■□■□■□■□");
    //println!("□■□■□■□■");

    println!("3 ⬛⬜⬛⬜⬛⬜⬛⬜");
    println!("4 ⬜⬛⬜⬛⬜⬛⬜⬛");
    println!("5 ⬛⬜⬛⬜⬛⬜⬛⬜");
    println!("6 ⬜⬛⬜⬛⬜⬛⬜⬛");

    println!("7 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟");
    println!("8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜\n");

    let b = crate::models::create_game();
    println!("{}", b);

    println!("⬛:{}", UnicodeWidthChar::width('⬛').unwrap());
    println!("♟:{}", UnicodeWidthChar::width('♟').unwrap());

    let route = warp::path("print")
        .and(with_db(b))
        .map(|board: Board| board.to_string());
    //? logging with
    //let route = warp::path("test").map(|| "").with(warp::log("chess"));

    //? serve
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}

//? including with database
fn with_db(db: Board) -> impl Filter<Extract = (Board,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

mod models {
    use std::fmt::Display;

    #[derive(Clone)]
    pub struct Board {
        board: Vec<Vec<GameObject>>,
    }

    impl Display for Board {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in &self.board {
                let r = row.iter().map(|item| item.symbol()).collect::<Vec<&str>>();
                for i in &r {
                    f.write_str(i);
                }
                f.write_str("\n");
            }

            Ok(())
        }
    }

    pub fn create_game() -> Board {
        let mut rows = Vec::with_capacity(8);
        let mut white_pawns: Vec<GameObject> = Vec::with_capacity(8);
        let mut black_pawns: Vec<GameObject> = Vec::with_capacity(8);
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
        let mut black_start: Vec<GameObject> = Vec::with_capacity(8);
        let mut white_start: Vec<GameObject> = Vec::with_capacity(8);

        for i in 0..9 {
            white_pawns.push(GameObject::WhitePawn(WhitePawn::new()));
            black_pawns.push(GameObject::BlackPawn(BlackPawn::new()));
            if i % 2 == 0 {
                black_start.push(GameObject::BlackTile(BlackTile {}));
                white_start.push(GameObject::WhiteTile(WhiteTile {}));
            } else {
                black_start.push(GameObject::WhiteTile(WhiteTile {}));
                white_start.push(GameObject::BlackTile(BlackTile {}));
            }
        }

        rows.push(black_pawns);

        rows.push(white_start.to_vec());
        rows.push(black_start.to_vec());
        rows.push(white_start.to_vec());
        rows.push(black_start.to_vec());
        rows.push(white_pawns);
        Board { board: rows }
    }

    trait Piece {
        fn symbol(&self) -> &'static str;
    }

    trait Tile {
        fn symbol(&self) -> &'static str;
    }
    #[derive(Clone)]
    enum GameObject {
        WhitePawn(WhitePawn),
        BlackPawn(BlackPawn),
        WhiteTile(WhiteTile),
        BlackTile(BlackTile),
    }

    impl Piece for GameObject {
        fn symbol(&self) -> &'static str {
            match self {
                GameObject::BlackPawn(val) => val.symbol(),
                GameObject::WhitePawn(val) => val.symbol(),
                GameObject::BlackTile(val) => val.symbol(),
                GameObject::WhiteTile(val) => val.symbol(),
            }
        }
    }
    #[derive(Clone)]
    struct WhiteTile {}
    impl Tile for WhiteTile {
        fn symbol(&self) -> &'static str {
            chess_backend::WHITE_TILE
        }
    }
    #[derive(Clone)]
    struct BlackTile {}
    impl Tile for BlackTile {
        fn symbol(&self) -> &'static str {
            chess_backend::BLACK_TILE
        }
    }
    #[derive(Clone)]
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
    }

    #[derive(Clone)]
    struct BlackPawn {
        movement: u16,
    }

    impl BlackPawn {
        fn new() -> Self {
            BlackPawn { movement: 1 }
        }
    }
    impl Piece for BlackPawn {
        fn symbol(&self) -> &'static str {
            chess_backend::BLACK_PAWN_SYMBOL
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
}

mod handlers {}
