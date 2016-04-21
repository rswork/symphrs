use super::event_dispatcher::*;
use super::event_dispatcher::event::*;
use super::event_dispatcher::event::EventStopable;

pub const KERNEL_EVENT_REQUEST: &'static str = "kernel.request";
pub const KERNEL_EVENT_EXCEPTION: &'static str = "kernel.exception";
pub const KERNEL_EVENT_VIEW: &'static str = "kernel.view";
pub const KERNEL_EVENT_CONTROLLER: &'static str = "kernel.controller";
pub const KERNEL_EVENT_RESPONSE: &'static str = "kernel.response";
pub const KERNEL_EVENT_TERMINATE: &'static str = "kernel.terminate";
pub const KERNEL_EVENT_FINISH_REQUEST: &'static str = "kernel.finish_request";

pub trait Terminable {
    fn terminate (&mut self, request: &mut String, response: &mut String);
}

pub trait Handleable {
    fn handle (&mut self, request: &mut String) -> String;
}

pub struct HttpKernel<D: Dispatchable<S>, S: EventStopable> {
    dispatcher: D,
}

impl <D: Dispatchable<S>, S: EventStopable> HttpKernel<D, S> {
    fn new (dispatcher: D) -> HttpKernel<D, S> {
        HttpKernel {dispatcher: dispatcher}
    }

    fn handle_raw(&mut self, request: &mut String) -> Result<&mut String, &mut String> {
        let mut request_event = Event::new();
        self.dispatcher.dispatch(KERNEL_EVENT_REQUEST, request_event);

        if request_event.has_response() {
            let mut response = request_event.get_response();
            return self.filter_response(response, request);
        }

        let mut controller_event = Event::new();
        self.dispatcher.dispatch(KERNEL_EVENT_CONTROLLER, controller_event);

        if controller_event.has_response() {
            let mut response = controller_event.get_response();
            return self.filter_response(response, request);
        } else {
            return self.handle_exception("No Controller found!".to_string(), request);
        }
    }

    fn handle_exception(&mut self, err: &mut String, request: &mut String) -> String {
        let mut event = Event::new();
        self.dispatcher.dispatch(KERNEL_EVENT_EXCEPTION, event);

        let mut response = "Error Request!".to_string();

        self.filter_response(response, request)
    }

    fn filter_response(&mut self, response: &mut String, request: &mut String) -> String {
        let response_event = Event::new();
        self.dispatcher.dispatch(KERNEL_EVENT_RESPONSE, response_event);

        response
    }

    fn finish_request(&mut self, request: &String) {
        let finish_event = Event::new();
        self.dispatcher.dispatch(KERNEL_EVENT_FINISH_REQUEST, finish_event);
    }
}

impl <D: Dispatchable<S>, S: EventStopable> Terminable for HttpKernel<D, S> {
    fn terminate (&mut self, request: &mut String, response: &mut String) {
        let mut event = Event::new();
        self.dispatcher.dispatch(KERNEL_EVENT_TERMINATE, event);
    }
}

impl<D: Dispatchable<S>, S: EventStopable> Handleable for HttpKernel<D, S> {
    fn handle (&mut self, request: &mut String) -> String {
        match self.handle_raw(request) {
            Ok(&mut response) => response,
            Err(&mut err) => self.handle_exception(err, request),
        }
    }
}
