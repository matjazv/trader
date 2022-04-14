use crate::database::Database;
use crate::User;
use rusqlite::Result;
use std::error::Error;

#[derive(Clone)]
pub struct UserTable {
    database: Database,
}

impl UserTable {
    pub fn new(database: Database) -> UserTable {
        UserTable { database }
    }

    pub fn add_user(&self, user: &User) -> bool {
        if self
            .database
            .get_connection()
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
        let connection = self.database.get_connection();
        let mut statement = connection.prepare("SELECT * FROM user WHERE account = ?;")?;
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
