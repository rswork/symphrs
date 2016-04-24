pub trait EventStopable {
    fn is_propagation_stopped(&self) -> bool;
    fn stop_propagation(&mut self);
}

pub struct Event {
    propagation_stopped: bool,
}

impl Event {
    pub fn new() -> Event {
        Event {propagation_stopped: false}
    }
}

impl EventStopable for Event {
    fn is_propagation_stopped(&self) -> bool {
        self.propagation_stopped
    }

    fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }
}

pub struct GetResponseEvent<'a> {
    propagation_stopped: bool,
    pub request: &'a mut String,
    pub response: Option<&'a mut String>,
}

impl<'a> GetResponseEvent<'a> {
    pub fn new(request: &mut String) -> GetResponseEvent {
        GetResponseEvent{
            propagation_stopped: false,
            request: request,
            response: None,
        }
    }
}

impl<'a> EventStopable for GetResponseEvent<'a> {
    fn is_propagation_stopped(&self) -> bool {
        self.propagation_stopped
    }

    fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_event_stopable<T: EventStopable>(event: &mut T) -> bool {
        assert_eq!(false, event.is_propagation_stopped());
        event.stop_propagation();
        assert_eq!(true, event.is_propagation_stopped());

        true
    }

    #[test]
    fn test_event() {
        let mut base_event = Event::new();
        test_event_stopable(&mut base_event);

        let mut request = "ping".to_string();
        let mut get_response_event = GetResponseEvent::new(&mut request);
        test_event_stopable(&mut get_response_event);
    }
}
