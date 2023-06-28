mod models;
use log::{info, warn};
use models::{get_diagonals, get_horse, get_orthogonals, get_pawn_movement, Board, Position};

use unicode_width::UnicodeWidthChar;
use warp::Filter;

#[tokio::main]
async fn main() {
    const RUST_LOG: &str = "RUST_LOG";
    if std::env::var_os(RUST_LOG).is_none() {
        std::env::set_var(RUST_LOG, "chess=info");
    }
    pretty_env_logger::init();
    /* println!("\n  A B C D E F G H");
    println!("1 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖");
    println!("2 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙");

    println!("3 ⬛⬜⬛⬜⬛⬜⬛⬜");
    println!("4 ⬜⬛⬜⬛⬜⬛⬜⬛");
    println!("5 ⬛⬜⬛⬜⬛⬜⬛⬜");
    println!("6 ⬜⬛⬜⬛⬜⬛⬜⬛");

    println!("7 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟");
    println!("8 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜\n");
    */
    let db = models::create_game();

    //println!("⬛:{}", UnicodeWidthChar::width('⬛').unwrap());
    //println!("♟:{}", UnicodeWidthChar::width('♟').unwrap());

    let route = warp::path("print")
        .and(with_db(db.clone()))
        .map(|board: Board| {
            println!("{}", board);
            board.to_string()
        });

    let move_path = warp::get().and(
        warp::path!("move" / Position / Position)
            .and(with_db(db.clone()))
            .map(|start: Position, end: Position, db: Board| {
                println!("start:{:?} - end:{:?} ", start, end);

                if let Err(error_message) = db.move_piece(&start, &end) {
                    return error_message;
                }

                println!("{}", db);
                format!("{:?} - {:?}\n{}", start, end, db)
            }),
    );

    let show_move = warp::get().and(warp::path!("show" / Position).and(with_db(db.clone())).map(
        |pos: Position, db: Board| {
            db.print_with_marked(&pos);
            let positions = db.show_moves_of_tile(&pos);
            format!("{:?}", positions)
        },
    ));
    //? printing to board for debugging
    db.print_with_marked(&Position::new_from_index(5, 0));

    let horse = get_pawn_movement(Position::new('a', 4), 1);
    println!("{:?}", horse.len());
    for i in horse {
        println!("{:?}", i);
    }
    //? serve
    warp::serve(move_path.or(route.or(show_move)).with(warp::log("chess")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

//? including with database
fn with_db(db: Board) -> impl Filter<Extract = (Board,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

mod handlers {}
