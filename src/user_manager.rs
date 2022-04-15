use crate::User;
use crate::UserTable;
use std::error::Error;

#[derive(Clone)]
pub struct UserManager {
    pub user_table: UserTable,
}

impl UserManager {
    pub fn new(user_table: &UserTable) -> UserManager {
        UserManager {
            user_table: user_table.clone(),
        }
    }

    pub fn add_user(&mut self, user: &User) -> Result<(), Box<dyn Error>> {
        if self.user_table.get_user(user.account()).is_ok() {
            return Err("User already exists. Can not add it.".into());
        }

        match self.user_table.add_user(user) {
            true => Ok(()),
            false => Err("Database error occurs. Can not add a new user.".into()),
        }
    }

    #[allow(dead_code)]
    pub fn update_user(&mut self, user: &User) -> Result<(), Box<dyn Error>> {
        if self.user_table.get_user(user.account()).is_err() {
            return Err("User does not exist. Can not update it.".into());
        }

        match self.user_table.update_user(user) {
            true => Ok(()),
            false => Err("Database error occurs. Can not update an user.".into()),
        }
    }

    pub fn get_user(&self, account: &str) -> Option<User> {
        match self.user_table.get_user(account) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
}
