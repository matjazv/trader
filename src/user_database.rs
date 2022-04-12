use crate::User;
use rusqlite::{params, Connection, Result, NO_PARAMS};

pub struct UserDatabase {
    connection: Connection,
}

impl UserDatabase {
    pub fn connect() -> Option<UserDatabase> {
        let connection = Connection::open_in_memory();
        match connection {
            Ok(connection) => Some(UserDatabase { connection }),
            Err(_) => None,
        }
    }

    pub fn create_table(&self) {
        self.connection
            .execute(
                "CREATE TABLE IF NOT EXISTS user (
                        id INTEGER PRIMARY KEY,
                        account TEXT NOT NULL,
                        nickName TEXT NOT NULL
                      )",
                NO_PARAMS,
            )
            .unwrap();
    }

    pub fn add_user(&self, user: User) {
        self.connection
            .execute(
                "INSERT INTO user (account, nickName) VALUES (?1, ?2)",
                &[&user.account(), &user.nick_name()],
            )
            .unwrap();
    }

    pub fn get_user(&self, account: &str) -> Result<()> {
        let mut stmt = self.connection.prepare("SELECT * FROM user")?;
        let person_iter = stmt.query_map([], |row| {
            let account: String = row.get(1)?;
            let nick_name: String = row.get(2)?;
            let mut user = User::new(&account);
            user.set_nick_name(&nick_name);
            Ok(user)
        })?;

        for person in person_iter {
            println!("Found person {:?}", person.unwrap());
        }

        Ok(())
    }
}
