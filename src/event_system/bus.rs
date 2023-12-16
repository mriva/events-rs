use std::{
    ops::Deref,
    sync::{Mutex, OnceLock},
};

use super::{events::Event, subscribers::Subscriber};

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

pub fn event_bus() -> &'static EventBus {
    static EVENT_BUS: OnceLock<EventBus> = OnceLock::new();

    EVENT_BUS.get_or_init(|| EventBus {
        subscribers: Mutex::new(Vec::new()),
    })
}
