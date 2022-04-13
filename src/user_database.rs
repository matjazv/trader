use crate::User;
use lazy_static::lazy_static;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use std::error::Error;

pub type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

lazy_static! {
    pub static ref SQLITEPOOL: SqlitePool = {
        let sqlite_database = "trader.db";
        let manager = SqliteConnectionManager::file(&sqlite_database);
        let pool = r2d2::Pool::builder().build(manager).unwrap();

        pool
    };
}

#[derive(Clone, Debug)]
pub struct UserDatabase {}

impl UserDatabase {
    pub fn create_tables(&self) -> bool {
        let conn = SQLITEPOOL.get().unwrap();
        if conn
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

    pub fn add_user(&self, user: &User) -> bool {
        let conn = SQLITEPOOL.get().unwrap();
        if conn
            .execute(
                "INSERT INTO user (account, nickName) VALUES (?1, ?2)",
                &[&user.account(), &user.nick_name()],
            )
            .is_err()
        {
            return false;
        }

        true
    }

    pub fn get_user(&self, account: &str) -> Result<User, Box<dyn Error>> {
        let conn = SQLITEPOOL.get().unwrap();
        let mut statement = conn.prepare("SELECT * FROM user WHERE account = ?;")?;
        let mut person_iter = statement.query_map([account], |row| {
            let account: String = row.get(1)?;
            let nick_name: String = row.get(2)?;
            let mut user = User::new(&account);
            user.set_nick_name(&nick_name);
            Ok(user)
        })?;

        if let Some(person) = person_iter.next() {
            return Ok(person.unwrap());
        }

        Err("No user found".into())
    }
}
