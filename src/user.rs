#[derive(Clone, Debug)]
pub struct User {
    address: String,
    nick_name: String,
}

impl User {
    pub fn new(address: &str) -> User {
        User {
            address: address.to_string(),
            nick_name: "".to_string(),
        }
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    #[allow(dead_code)]
    pub fn nick_name(&self) -> &String {
        &self.nick_name
    }
}
