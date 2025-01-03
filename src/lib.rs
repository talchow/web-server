// Worker
// create a struct Worker to store the thread and thread's id 
// the Worker::new() take a usize(the usize is the number which would be create),and a receiver as parameter
// the receiver is the type alia of task that send into the task-queue,which is wraped by the Arc<Mutex<mpsc::Receiver<>>>
// and call the thread::spawn method to generate a thread,and return a Worker instance
// 
// ThreadPool::new()
// First,assert the thread number is not less than 0
// Second,call the mpsc::channel() to create a channel which is used to pass task 
// and wrap the receiver with Arc<Mutex<>>,make sure the receiver has one owner at one time
// Third,create a container workers to store the Worker(or called thread)
// and push these Worker into the vector
// At last,return the ThreadPool instance which contain thread and sender

// the closure's format 
// new_closure = | x | x + 3
// the x in the || is capture from the environment ,and after the x + 3 after the || ,is how to handle the x
use std::sync::{mpsc, Arc,Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender:mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // workers.push(Worker::new(id, Arc::clone(&receiver)));
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool { workers, sender }
    }

    pub fn excute<F>(&self, f:F)
    where 
        F:FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            // self.sender.send(job).unwrap();
            mpsc::Sender::send(&self.sender, job).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
           let job = receiver.lock().unwrap().recv().unwrap();

           println!("Worker {id} got a job; executing.");

           job();
        });

        Worker { id, thread }
    }
}
