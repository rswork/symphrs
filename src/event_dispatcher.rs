pub mod event {
    trait EventStopable {
        fn is_propagation_stopped(&self) -> bool {
            self.propagation_stopped
        }

        fn stop_propagation(&mut self) {
            self.propagation_stopped = true;
        }
    }

    pub struct Event {
        propagation_stopped: bool,
    }

    impl EventStopable for Event {
    }

    impl Event {
        fn new () -> Event {
            Event {propagation_stopped: false}
        }
    }

    pub struct GetResponseEvent {
        propagation_stopped: bool,
        response: String,
    }

    impl GetResponseEvent {
        fn new(response: &mut String) -> GetResponseEvent {
            GetResponseEvent{propagation_stopped: false, response: response}
        }
    }

    impl EventStopable for GetResponseEvent {
    }
}

use self::event::{EventStopable, Event};
use std::collections::HashMap;

pub trait ListenerCallable {
    fn call(&self, event_name: &str, event: &mut EventStopable);
}

pub struct EventListener {
    callback: fn(event_name: &str, event: EventStopable),
}

impl ListenerCallable for EventListener {
    fn call (&self, event_name: &str, event: &mut EventStopable) {
        self.callback(event);
    }
}

impl EventListener {
    fn new (callback: fn(event: EventStopable)) -> EventListener {
        EventListener {callback: callback}
    }
}

pub trait Dispatchable {
    fn dispatch (&self, event_name: &str, event: &mut EventStopable);
}

pub struct EventDispatcher<'a> {
    listeners: &'a mut HashMap<String, Vec<EventListener>>,
}

impl<'a> EventDispatcher<'a> {
    fn new() -> &'a mut EventDispatcher<'a> {
        EventDispatcher {listeners: HashMap::new()}
    }
}

impl<'a> Dispatchable for EventDispatcher<'a> {
    fn dispatch<'b> (&self, event_name: &str, event: &mut EventStopable) {
        if let Some(listeners) = self.listeners.get(event_name) {
            for listener in listeners {
                listener.call(event_name, event);

                if event.is_propagation_stopped == false {
                    break;
                }
            }
        }
    }
}
