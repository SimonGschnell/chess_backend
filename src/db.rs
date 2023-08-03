use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

const DB_URL: &str = "db/chess.db";

pub struct DB {
    connection: Pool<Sqlite>,
}

impl DB {
    pub async fn db_start() -> Self {
        let connection = db_migrate().await;
        let result = sqlx::query!("select * from piece_colors;")
            .fetch_all(&connection)
            .await
            .unwrap();
        println!("{:?}", result);
        DB { connection }
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
