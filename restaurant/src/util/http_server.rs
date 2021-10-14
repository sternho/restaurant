use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

impl ThreadPool {
    pub fn new(total_threads: usize) -> ThreadPool {
        assert!(total_threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(total_threads);

        for id in 0..total_threads {
            // workers[id] = Worker::new(id, Arc::clone(&receiver));
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, closure: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(closure);
        // let job = Box::new(Message::NewJob(closure));

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker {id, thread: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        // println!("Worker id {} got a job, executing!", id);
                        job.call_box();
                    },
                    Message::Terminate => break,
                }
            })),
        }
    }
}

type Job = Box<dyn FnBox + Send + 'static>;