use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
	NewJob(Job),
	Terminate,
}

trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self: Box<F>) {
		(*self)()
	}
}
type Job = Box<FnBox + Send + 'static>;

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

impl ThreadPool {
	/// create a new ThreadPool.
	/// the size is the number of threads in the pool
	/// #Panics 
	///if size is less or equal to zero, `new` panics
	
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		ThreadPool {
			workers,
			sender,
		}
	}

	pub fn execute<F> (&self, f: F)
		where 
			F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);
		self.sender.send(Message::NewJob(job)).unwrap();
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		println!("Sending terminate message to all workers");
		for _ in &mut self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		println!("Shutting down all workers.");

		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);
			self.sender.send(Message::Terminate).unwrap();

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				let message = receiver.lock().unwrap().recv().unwrap();

				match message {
					Message::NewJob(job) => {
						println!("Worker {} got a job; executing.", id);
						job.call_box();
					},
					Message::Terminate => {
						println!("Worker {} was told to terminate.", id);
						break;
					},
				}
				

				
			}
		});
		Worker {
			id,
			thread: Some(thread),
		}
	}
}


/////////////////// Partial Implementation for new() when return value
/////////////////// is Result<ThreadPool, PoolCreationError>
/*
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PoolCreationError;

impl Error for PoolCreationError {
	fn description(&self) -> &str {
		"ThreadPool size must be greater than zero"
	}
}

impl fmt::Display for PoolCreationError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "PoolCreationError: ThreadPool size must be > zero")
	}
}

pub struct ThreadPool;

impl ThreadPool {
	/// create a new ThreadPool.
	/// the size is the number of threads in the pool
	/// #Panics 
	///if size is less or equal to zero, `new` panics
	/*
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);
		ThreadPool
	}
	*/

	pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
		if size == 0 {
			return Err(PoolCreationError);
		}
		 Ok(ThreadPool)
	}
*/