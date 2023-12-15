use events::{event_bus, Event, Logger, Mailer, Subscriber, UserRegistered};

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
