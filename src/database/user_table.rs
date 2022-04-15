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
                "INSERT INTO user (account, nickName) VALUES (?1, ?2);",
                &[user.account(), user.nick_name()],
            )
            .is_err()
        {
            return false;
        }

        true
    }

    #[allow(dead_code)]
    pub fn update_user(&self, user: &User) -> bool {
        if self
            .database
            .get_connection()
            .execute(
                "UPDATE user SET nickName = ? WHERE account = ?;",
                &[user.nick_name(), user.account()],
            )
            .is_err()
        {
            return false;
        }

        true
    }

    pub fn get_user(&self, account: &str) -> Result<User, Box<dyn Error>> {
        let user = self.database.get_connection().query_row(
            "SELECT account, nickName FROM user WHERE account = ?;",
            [account],
            |row| {
                let account: String = row.get(0)?;
                let nick_name: String = row.get(1)?;
                Ok(User::new(&account, &nick_name))
            },
        );

        match user {
            Ok(user) => Ok(user),
            Err(_) => Err("No user found".into()),
        }
    }
}
