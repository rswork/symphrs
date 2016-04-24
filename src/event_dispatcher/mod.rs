pub mod event;

use self::event::EventStopable;
use std::collections::HashMap;

pub trait ListenerCallable: PartialEq {
    fn call(&self, event_name: &str, event: &mut EventStopable);
}

pub struct EventListener {
    callback: fn(event_name: &str, event: &mut EventStopable),
}

impl EventListener {
    pub fn new (callback: fn(event_name: &str, event: &mut EventStopable)) -> EventListener {
        EventListener {callback: callback}
    }
}

impl ListenerCallable for EventListener {
    fn call (&self, event_name: &str, event: &mut EventStopable) {
        let callback = self.callback;
        callback(event_name, event);
    }
}

impl PartialEq for EventListener {
    fn eq(&self, other: &EventListener) -> bool {
        (self.callback as *const()) == (other.callback as *const())
    }

    fn ne(&self, other: &EventListener) -> bool {
        !self.eq(other)
    }
}

pub trait Dispatchable<S> where S: EventStopable {
    fn dispatch (&self, event_name: &str, event: &mut S);
}

pub struct EventDispatcher<'a, L> where L: 'a + ListenerCallable {
    listeners: HashMap<&'a str, Vec<&'a L>>,
}

impl<'a, L: 'a + ListenerCallable> EventDispatcher<'a, L> {
    pub fn new() -> EventDispatcher<'a, L> {
        EventDispatcher{listeners: HashMap::new()}
    }

    pub fn add_listener(&mut self, event_name: &'a str, listener: &'a L) {
        if !self.listeners.contains_key(event_name) {
            self.listeners.insert(event_name, Vec::new());
        }

        if let Some(mut listeners) = self.listeners.get_mut(event_name) {
            listeners.push(listener);
        }
    }

    pub fn remove_listener(&mut self, event_name: &'a str, listener: &'a mut L) {
        if self.listeners.contains_key(event_name) {
            if let Some(mut listeners) = self.listeners.get_mut(event_name) {
                match listeners.iter().position(|x| *x == listener) {
                    Some(index) => {
                        listeners.remove(index);
                    },
                    _ => {},
                }
            }
        }
    }
}

impl<'a, S: 'a + EventStopable> Dispatchable<S> for EventDispatcher<'a, EventListener> {
    fn dispatch(&self, event_name: &str, event: &mut S) {
        if let Some(listeners) = self.listeners.get(event_name) {
            for listener in listeners {
                listener.call(event_name, event);

                if !event.is_propagation_stopped() {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::event::*;

    fn print_event_info(event_name: &str, event: &mut EventStopable) {
        println!("callback from event: {}", event_name);

        event.stop_propagation();
    }

    #[test]
    fn test_dispatcher() {
        let event_name = "test_a";
        let mut event = Event::new();
        let callback_one: fn(event_name: &str, event: &mut EventStopable) = print_event_info;
        let mut listener_one = EventListener::new(callback_one);
        let mut dispatcher = EventDispatcher::new();

        dispatcher.dispatch(event_name, &mut event);
        assert_eq!(false, event.is_propagation_stopped());
        dispatcher.dispatch(event_name, &mut event);
        assert_eq!(false, event.is_propagation_stopped());

        dispatcher.add_listener(event_name, &mut listener_one);
        dispatcher.dispatch(event_name, &mut event);
        assert_eq!(true, event.is_propagation_stopped());
    }
}
