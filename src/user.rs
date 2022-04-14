#[derive(Clone, Debug)]
pub struct User {
    pub account: String,
    pub nick_name: String,
}

impl User {
    pub fn new(account: &str) -> User {
        User {
            account: account.to_ascii_lowercase(),
            nick_name: "".to_string(),
        }
    }
}
