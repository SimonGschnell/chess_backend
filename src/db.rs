use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

const DB_URL: &str = "db/chess.db";

pub struct DB {
    connection: Pool<Sqlite>,
}

impl DB {
    pub async fn db_start() -> Self {
        db_check().await;
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
    match sqlx::migrate!("./migrations").run(&connection).await {
        Err(err) => {
            panic!("{err}");
        }
        Ok(_) => {
            println!("migration successfull");
        }
    }
    connection
}
