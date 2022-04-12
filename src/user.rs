#[derive(Clone, Debug)]
pub struct User {
    account: String,
    nick_name: String,
}

impl User {
    pub fn new(account: &str) -> User {
        User {
            account: account.to_ascii_lowercase(),
            nick_name: "".to_string(),
        }
    }

    pub fn account(&self) -> &String {
        &self.account
    }

    #[allow(dead_code)]
    pub fn nick_name(&self) -> &String {
        &self.nick_name
    }
}
