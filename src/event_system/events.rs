#[derive(Debug)]
pub enum Event {
    UserRegistered(UserRegistered),
    UserDeleted(UserDeleted),
}

#[derive(Debug)]
pub struct UserRegistered {
    pub payload: String,
}

impl UserRegistered {
    fn get_payload(&self) -> String {
        self.payload.clone()
    }
}

#[derive(Debug)]
pub struct UserDeleted {
    pub payload: String,
}

impl UserDeleted {
    fn get_payload(&self) -> String {
        self.payload.clone()
    }
}
