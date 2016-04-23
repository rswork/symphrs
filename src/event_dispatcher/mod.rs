pub mod event;

use self::event::EventStopable;
use std::collections::HashMap;

pub trait ListenerCallable {
    fn call(&self, event_name: &str, event: &mut EventStopable);
}

pub struct EventListener {
    callback: fn(event_name: &str, event: &mut EventStopable),
}

impl ListenerCallable for EventListener {
    fn call (&self, event_name: &str, event: &mut EventStopable) {
        let callback = self.callback;
        callback(event_name, event);
    }
}

impl EventListener {
    fn new (callback: fn(event_name: &str, event: &mut EventStopable)) -> EventListener {
        EventListener {callback: callback}
    }
}

pub trait Dispatchable<S> where S: EventStopable {
    fn dispatch (&self, event_name: &str, event: &mut S);
}

pub struct EventDispatcher<'a, String: 'a, EventListener: 'a>{
    listeners: HashMap<String, Vec<&'a EventListener>>,
}

impl<'a> EventDispatcher<'a, String, EventListener> {
    fn new() -> EventDispatcher<'a, String, EventListener> {
        EventDispatcher{listeners: HashMap::new()}
    }
}

impl<'a, S: EventStopable> Dispatchable<S> for EventDispatcher<'a, String, EventListener> {
    fn dispatch(&self, event_name: &str, event: &mut S) {
        if let Some(listeners) = self.listeners.get(event_name) {
            for listener in listeners {
                listener.call(event_name, event);

                if event.is_propagation_stopped() == false {
                    break;
                }
            }
        }
    }
}
