use std::{
    ops::Deref,
    sync::{Mutex, OnceLock},
};

pub trait Subscriber {
    fn handle(&self, event: &Event);
}

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

pub struct EventBus {
    pub subscribers: Mutex<Vec<Box<dyn Subscriber + Send>>>,
}

impl EventBus {
    pub fn register_subscriber(&self, subscriber: Box<dyn Subscriber + Send>) {
        self.subscribers.lock().unwrap().push(subscriber);
    }

    pub fn publish(&self, event: Event) {
        for subscriber in self.subscribers.lock().unwrap().deref() {
            subscriber.handle(&event);
        }
    }
}

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
pub fn event_bus() -> &'static EventBus {
    static EVENT_BUS: OnceLock<EventBus> = OnceLock::new();

    EVENT_BUS.get_or_init(|| EventBus {
        subscribers: Mutex::new(Vec::new()),
    })
}
