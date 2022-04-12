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

    pub fn add_user(&mut self, account: &str) -> Result<User, Box<dyn Error>> {
        let account = account.to_ascii_lowercase();
        if self.users.read().contains_key(&account) {
            return Err("User already exists.".into());
        }

        let user = User::new(&account);
        self.users.write().insert(account, user.clone());
        Ok(user)
    }

    pub fn get_user(&self, account: &String) -> Option<User> {
        self.users.read().get(account).cloned()
    }
}
