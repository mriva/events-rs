pub struct EventBus {
    pub subscribers: Vec<Box<dyn Subscriber>>,
}

impl EventBus {
    pub fn subscribe(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }

    pub fn publish(&self, event: Box<dyn Event>) {
        for subscriber in &self.subscribers {
            subscriber.handle(&event);
        }
    }
}

pub trait Event {
    fn payload(&self) -> String;
    fn name(&self) -> &'static str;
}

pub trait Subscriber {
    fn handle(&self, event: &Box<dyn Event>);
}

#[derive(Debug)]
pub struct UserRegistered {
    pub payload: String,
}

impl Event for UserRegistered {
    fn payload(&self) -> String {
        format!("UserRegistered: {}", self.payload)
    }
    fn name(&self) -> &'static str {
        "UserRegistered"
    }
}

#[derive(Debug)]
pub struct UserDeleted {
    payload: String,
}

impl Event for UserDeleted {
    fn payload(&self) -> String {
        format!("UserDeleted: {}", self.payload)
    }
    fn name(&self) -> &'static str {
        "UserDeleted"
    }
}

pub struct Mailer {}

impl Subscriber for Mailer {
    fn handle(&self, event: &Box<dyn Event>) {
        match event.name() {
            "UserRegistered" => self.handle_user_was_registed(event),
            "UserDeleted" => self.handle_user_was_deleted(event),
            _ => {}
        }
    }
}

impl Mailer {
    fn handle_user_was_registed(&self, event: &Box<dyn Event>) {
        // ....
    }

    fn handle_user_was_deleted(&self, event: &Box<dyn Event>) {
        // ....
    }
}
