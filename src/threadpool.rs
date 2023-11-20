use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    usize,
};

pub struct ThreadPool {
    sender: Sender<Job>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn with_capacaty(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool { sender, workers }
    }

    pub fn submit(&self, task: FnPool) {
        self.sender.send(Job::Task(task)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Job::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            let _ = worker.thread.take().unwrap().join();
        }
    }
}

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F> FnBox for F
where
    F: FnOnce(),
{
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub type FnPool = Box<dyn FnBox + Send + 'static>;
enum Job {
    Task(FnPool),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let task = receiver.lock().unwrap().recv().unwrap();
            match task {
                Job::Task(task) => {
                    println!("Worker {} got a job; executing.", id);
                    task.call_box();
                }
                Job::Terminate => {
                    println!("Worker {} terminating.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

mod test {
    use std::{
        sync::{
            atomic::{AtomicU8, Ordering},
            Arc,
        },
        thread::{self},
        time::Duration,
    };

    use super::ThreadPool;

    #[test]
    fn thread_works() {
        let count = Arc::new(AtomicU8::new(0));
        let pool = ThreadPool::with_capacaty(10);

        for _ in 0..10 {
            let count = Arc::clone(&count);
            let task = Box::new(move || {
                count.fetch_add(1, Ordering::SeqCst);
                println!("Hello World");
            });
            pool.submit(task);
        }

        thread::sleep(Duration::from_secs(2));
        assert_eq!(10, count.fetch_or(0, Ordering::SeqCst));
    }
}
