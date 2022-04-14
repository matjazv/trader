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

    pub fn add_user(&mut self, account: &str) -> Result<User, Box<dyn Error>> {
        let account = account.to_ascii_lowercase();
        if self.user_table.get_user(&account).is_ok() {
            return Err("User already exists.".into());
        }

        let user = User::new(&account);
        self.user_table.add_user(&user);
        Ok(user)
    }

    pub fn get_user(&self, account: &String) -> Option<User> {
        match self.user_table.get_user(account) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
}
