use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

const DB_URL: &str = "db/chess.db";

pub struct DB {
    pub connection: Pool<Sqlite>,
}

impl DB {
    pub async fn db_start() -> Self {
        db_check().await;
        let connection = db_migrate().await;
        DB { connection }
    }
}

async fn db_check() {
    //?create db
    if !sqlx::Sqlite::database_exists(DB_URL).await.unwrap() {
        println!("creating db at {DB_URL}");
        match Sqlite::create_database(DB_URL).await {
            Err(err) => {
                panic!("{err}");
            }
            Ok(_) => {
                println!("successfull creation of the DB at {DB_URL}");
            }
        }
    } else {
        println!("db found");
    }
}

async fn db_migrate() -> Pool<Sqlite> {
    let connection = SqlitePool::connect(DB_URL).await.unwrap();
    match sqlx::migrate!("db/migrations").run(&connection).await {
        Err(err) => {
            panic!("{err}");
        }
        Ok(_) => {
            println!("migration successfull");
        }
    }
    connection
}
