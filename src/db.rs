use std::collections::HashMap;

use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Row, Sqlite, SqlitePool};

use crate::models::printablePiece;

const DB_URL: &str = "db/chess.db";

#[derive(Clone)]
pub struct DB {
    connection: Pool<Sqlite>,
    board_name: String,
}

impl DB {
    pub async fn db_start() -> Self {
        let connection = db_migrate().await;
        let result = sqlx::query!("select * from piece_colors;")
            .fetch_all(&connection)
            .await
            .unwrap();
        println!("{:?}", result);
        DB {
            connection,
            board_name: String::from("board"),
        }
    }

    pub async fn print(&self) -> HashMap<i8, Vec<printablePiece>> {
        let board_query =
            "select col,row,symbol from board join pieces on (color,name) = (piece_color,piece_name);"
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

    pub async fn query(&self) {
        let stuff = sqlx::query!("SELECT * FROM piece_colors ")
            .fetch_all(&self.connection)
            .await
            .unwrap();
        println!("{:?}", stuff);
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
