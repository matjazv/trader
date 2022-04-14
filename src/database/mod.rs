pub mod user_table;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

#[derive(Clone, Debug)]
pub struct Database {
    connection: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn init() -> Database {
        let sqlite_database = "trader.db";
        let manager = SqliteConnectionManager::file(&sqlite_database);
        let pool = r2d2::Pool::builder().build(manager).unwrap();

        let database = Database { connection: pool };
        database.create_tables();
        database
    }

    pub fn get_connection(&self) -> PooledConnection<SqliteConnectionManager> {
        self.connection.get().unwrap()
    }

    fn create_tables(&self) -> bool {
        if self
            .get_connection()
            .execute(
                "CREATE TABLE IF NOT EXISTS user (
                        id INTEGER PRIMARY KEY,
                        account TEXT NOT NULL,
                        nickName TEXT NOT NULL
                      )",
                [],
            )
            .is_err()
        {
            return false;
        }

        true
    }
}
