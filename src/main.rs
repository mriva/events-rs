use events::{event_bus, Event, Subscriber};

pub struct Notifier {}
impl Subscriber for Notifier {
    fn handle(&self, event: &Box<dyn Event>) {
        println!("Notifier: {:?}", event.get_payload());
    }
}

pub struct Mailer {}
impl Subscriber for Mailer {
    fn handle(&self, event: &Box<dyn Event>) {
        println!("Mailer: {:?}", event.get_payload());
    }
}

pub struct Logger {}
impl Subscriber for Logger {
    fn handle(&self, event: &Box<dyn Event>) {
        println!("Logger: {:?}", event.get_payload());
    }
}

#[derive(Debug)]
pub struct UserRegistered {
    pub payload: String,
}

impl Event for UserRegistered {
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
fn main() {
    event_bus().register_subscriber(Box::new(Mailer {}));
    event_bus().register_subscriber(Box::new(Logger {}));
    event_bus().register_subscriber(Box::new(Notifier {}));

    event_bus().publish(Box::new(UserRegistered {
        payload: "User Registered".to_string(),
    }));
}
