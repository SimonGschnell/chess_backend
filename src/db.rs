use std::cell::{Ref, RefCell};
use std::collections::HashMap;

use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Row, Sqlite, SqlitePool};

use crate::models::{printablePiece, Board, Color, GameObject, Position, Tile};
type Matrix = Vec<Vec<RefCell<Tile>>>;
const DB_URL: &str = "db/chess.db";

#[derive(Clone)]
pub struct DB {
    connection: Pool<Sqlite>,
    board_name: String,
}

impl DB {
    pub async fn db_start() -> Self {
        let connection = db_migrate().await;

        DB {
            connection,
            board_name: String::from("board"),
        }
    }

    pub async fn print(&self) -> HashMap<i8, Vec<printablePiece>> {
        let board_query =
            "select col,row,symbol,field_color from board left join pieces on (color,name) = (piece_color,piece_name);"
                .replace("board", &self.board_name);
        let board = sqlx::query(&board_query)
            .fetch_all(&self.connection)
            .await
            .unwrap();

        let mut res = HashMap::new();

        for i in 1..=8 {
            res.insert(i, Vec::with_capacity(8));
        }
        for r in board {
            let piece = printablePiece::from_row(&r).unwrap();

            res.get_mut(&piece.row).unwrap().push(piece);
        }
        res
    }

    pub async fn get_board(&self) -> Board {
        let chess_board_query = "select player_turn from chess_board where board_name =?;";
        let player_turn = sqlx::query(chess_board_query)
            .bind(&self.board_name)
            .fetch_one(&self.connection)
            .await
            .unwrap();
        let board_query = "select field_color,piece_color as color ,piece_name as name,range,row  from board left join pieces ON (color,name)=(piece_color,piece_name);".replace("board", &self.board_name);
        let pieces = sqlx::query(&board_query)
            .fetch_all(&self.connection)
            .await
            .unwrap();
        let player_turn = match player_turn.try_get("player_turn") {
            Ok("WHITE") => Color::White,
            Ok("BLACK") => Color::Black,
            _ => panic!("Invalid player_turn_color"),
        };
        let mut res: Matrix = Vec::with_capacity(8);
        for _ in 1..=8 {
            res.push(Vec::with_capacity(8));
        }

        pieces
            .iter()
            .for_each(|row| match row.try_get("row").unwrap() {
                row_index @ 1..=8 => {
                    let a: i32 = row_index;
                    res[a as usize - 1].push(RefCell::new(Tile::from_row(row).unwrap()));
                }
                _ => panic!("Unexpected row"),
            });

        Board {
            board: res,
            players_turn: player_turn,
        }
    }

    pub async fn move_piece(
        &self,
        from: Position,
        to: Position,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let from_piece = "select piece_color, piece_name from board where row =? and col =?";
        let from_piece = sqlx::query(from_piece)
            .bind(from.rank)
            .bind(String::from(from.file))
            .fetch_one(&self.connection)
            .await?;
        let from_piece_name: String = from_piece.try_get("piece_name")?;
        let from_piece_color: String = from_piece.try_get("piece_color")?;
        let empty_from_piece =
            "update board set has_piece=0, piece_color =NULL, piece_name =NULL where row =? and col =?";
        sqlx::query(empty_from_piece)
            .bind(from.rank)
            .bind(String::from(from.file))
            .execute(&self.connection)
            .await?;
        let move_query = "update board set piece_color =?, piece_name =? where row =? and col =?";
        sqlx::query(move_query)
            .bind(from_piece_color)
            .bind(from_piece_name)
            .bind(to.rank)
            .bind(String::from(to.file))
            .execute(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn query(&self) {
        /* let stuff = sqlx::query!("SELECT * FROM piece_colors ")
            .fetch_all(&self.connection)
            .await
            .unwrap();
        println!("{:?}", stuff); */
    }
}

async fn db_migrate() -> Pool<Sqlite> {
    let connection = match SqlitePool::connect(DB_URL).await {
        Ok(pool) => pool,
        Err(err) => {
            panic!("{err}");
        }
    };
    match sqlx::migrate!("./db/migrations").run(&connection).await {
        Err(err) => {
            panic!("{err}");
        }
        Ok(_) => {
            println!("migration successfull");
        }
    }
    connection
}
