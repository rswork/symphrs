use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

type PoolCreatingError = String;

pub struct JobResult {
    request: String,
    response: String,
}

impl JobResult {
    pub fn new(request: String, response: String) -> JobResult {
        JobResult { request, response }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    job_result_sender: mpsc::Sender<WatcherMessage>,
    watchers: Vec<Watcher>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreatingError> {
        if size == 0 {
            let size = 1;
        }

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        let mut watchers = Vec::with_capacity(size);
        let (job_result_sender, job_result_receiver) = mpsc::channel();
        let job_result_receiver = Arc::new(Mutex::new(job_result_receiver));

        for id in 0..size {
            watchers.push(Watcher::new(id, job_result_receiver.clone()));
        }

        Ok(ThreadPool {
            workers,
            sender,
            job_result_sender,
            watchers,
        })
    }

    pub fn execute<F>(&self, callback: F)
        where
            F: FnOnce() -> JobResult + Send + 'static
    {
        let job_result_sender = self.job_result_sender.clone();
        let job = Message::NewJob(Box::new(move || {
            let job_result: JobResult = callback();
            job_result_sender.send(WatcherMessage::NewJobResult(job_result));
        }));

        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("Shutting down all watchers.");

        for watcher in &mut self.watchers {
            self.job_result_sender.send(WatcherMessage::Terminate).unwrap();
        }

        for watcher in &mut self.watchers {
            println!("Shutting down watcher {}", watcher.id);

            if let Some(thread) = watcher.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>
    ) -> Worker {
        let thread = Some(thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                match job {
                    Message::NewJob(job) => {
                        println!("worker {} got a job!", id);
                        job.call_box();
                        println!("worker {} finished a job!", id);
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        }));

        Worker {
            id,
            thread,
        }
    }
}

enum WatcherMessage {
    NewJobResult(JobResult),
    Terminate,
}

struct Watcher {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Watcher {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<WatcherMessage>>>) -> Watcher {
        let thread = Some(thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                match job {
                    WatcherMessage::NewJobResult(job) => {
                        thread::sleep(Duration::from_secs(1));

                        println!("watcher {} got a job result!", id);
                        println!("---- new connetion established ----\n{}{}\n---- connection finished! ----", job.request, job.response);
                        println!("watcher {} finished a job result!", id);
                    },
                    WatcherMessage::Terminate => {
                        println!("watcher {} was told to terminate.", id);

                        break;
                    },
                }
            }
        }));

        Watcher {
            id,
            thread,
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
