#[derive(Clone, Debug)]
pub struct User {
    account: String,
    nick_name: String,
}

impl User {
    pub fn new(account: &str, nick_name: &str) -> User {
        User {
            account: account.to_ascii_lowercase(),
            nick_name: nick_name.to_string(),
        }
    }

    pub fn account(&self) -> &String {
        &self.account
    }

    pub fn nick_name(&self) -> &String {
        &self.nick_name
    }

    #[allow(dead_code)]
    pub fn set_nick_name(&mut self, nick_name: &str) {
        self.nick_name = nick_name.to_string();
    }
}
