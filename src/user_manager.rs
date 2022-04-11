use crate::User;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserManager {
    pub users: Arc<RwLock<HashMap<String, User>>>,
}

impl UserManager {
    pub fn init() -> UserManager {
        UserManager {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_user(&mut self, address: &str) -> Result<User, Box<dyn Error>> {
        let address = address.to_ascii_lowercase();
        if self.users.read().contains_key(&address) {
            return Err("User already exists.".into());
        }

        let user = User::new(&address);
        self.users.write().insert(address, user.clone());
        Ok(user)
    }

    #[allow(dead_code)]
    pub fn get_user(&self, address: &String) -> Result<User, Box<dyn Error>> {
        match self.users.read().get(address) {
            None => Err("User does not exist.".into()),
            Some(user) => Ok(user.clone()),
        }
    }
}
