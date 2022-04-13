use crate::User;
use crate::UserDatabase;
use std::error::Error;

#[derive(Clone)]
pub struct UserManager {
    pub database: UserDatabase,
}

impl UserManager {
    pub fn init(database: &UserDatabase) -> UserManager {
        UserManager {
            database: database.clone(),
        }
    }

    pub fn add_user(&mut self, account: &str) -> Result<User, Box<dyn Error>> {
        let account = account.to_ascii_lowercase();
        if self.database.get_user(&account).is_ok() {
            return Err("User already exists.".into());
        }

        let user = User::new(&account);
        self.database.add_user(&user);
        Ok(user)
    }

    pub fn get_user(&self, account: &String) -> Option<User> {
        match self.database.get_user(account) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
}
