use super::events::Event;

pub trait Subscriber {
    fn handle(&self, event: &Event);
}
