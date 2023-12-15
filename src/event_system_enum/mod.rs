pub struct EventBus {
    pub subscribers: Vec<Box<dyn Subscriber>>,
}

pub trait Subscriber {
    fn handle(&self, event: &Event);
}

pub struct Mailer {}
impl Subscriber for Mailer {
    fn handle(&self, event: &Event) {
        match event {
            Event::UserRegistered(event) => self.handle_user_was_registed(event),
            _ => {}
        }
    }
}

impl Mailer {
    fn handle_user_was_registed(&self, event: &UserRegistered) {
        println!("Sending mail to {}", event.get_payload());
    }
}

pub enum Event {
    UserRegistered(UserRegistered),
    UserDeleted(UserDeleted),
}

trait EventVariant {
    fn get_payload(&self) -> String;
}

pub struct UserRegistered {
    pub payload: String,
}

impl UserRegistered {
    fn get_payload(&self) -> String {
        self.payload.clone()
    }
}

pub struct UserDeleted {
    pub payload: String,
}

impl UserDeleted {
    fn get_payload(&self) -> String {
        self.payload.clone()
    }
}

impl EventBus {
    pub fn subscribe(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }

    pub fn publish(&self, event: Event) {
        for subscriber in &self.subscribers {
            subscriber.handle(&event);
        }
    }
}
