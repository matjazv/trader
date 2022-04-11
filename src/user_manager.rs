use crate::User;
use std::collections::HashMap;

pub struct UserManager {
    pub users: HashMap<String, User>,
}

impl UserManager {
    pub fn init() -> UserManager {
        UserManager {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, address: &str) -> Result<User, Box<dyn std::error::Error>> {
        if self.users.contains_key(address) {
            return Err("User already exists.".into());
        }

        let user = User::new(address);
        self.users.insert(address.to_string(), user.clone());
        Ok(user)
    }

    pub fn get_user(&self, address: &String) -> Result<User, Box<dyn std::error::Error>> {
        match self.users.get(address) {
            None => Err("User does not exist.".into()),
            Some(user) => Ok(user.clone()),
        }
    }
}
