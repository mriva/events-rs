use event_system::{event_bus, Event, Subscriber, UserRegistered};

mod event_system;

pub struct Mailer {}
impl Subscriber for Mailer {
    fn handle(&self, event: &Event) {
        println!("Mailer: {:?}", event);
    }
}

pub struct Logger {}
impl Subscriber for Logger {
    fn handle(&self, event: &Event) {
        println!("Logger: {:?}", event);
    }
}

pub struct Notifier {}
impl Subscriber for Notifier {
    fn handle(&self, event: &Event) {
        println!("Notifier: {:?}", event);
    }
}

fn main() {
    event_bus().register_subscriber(Box::new(Mailer {}));
    event_bus().register_subscriber(Box::new(Logger {}));
    event_bus().register_subscriber(Box::new(Notifier {}));

    event_bus().publish(Event::UserRegistered(UserRegistered {
        payload: "User Registered".to_string(),
    }));
}
