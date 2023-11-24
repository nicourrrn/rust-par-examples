pub mod single_server {

    use std::{
        fs,
        io::prelude::*,
        net::{TcpListener, TcpStream},
        thread::sleep,
        time::Duration,
    };

    pub fn run(with_sleep: bool) {
        let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
        for stream in listener.incoming() {
            handle_connection(stream.unwrap());
            if with_sleep {
                sleep(Duration::from_millis(100));
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("file.txt").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

pub mod mutli_threaded_server {
    use std::{
        sync::{mpsc, Arc, Mutex},
        thread,
    };

    use std::fs;
    use std::io::prelude::*;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::thread::sleep;
    use std::time::Duration;

    pub fn run(pool_size: usize, with_sleep: bool) {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        let pool = ThreadPool::new(pool_size);

        for stream in listener.incoming() {
            pool.execute(move || {
                handle_connection(stream.unwrap());
                if with_sleep {
                    sleep(Duration::from_millis(100));
                }
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("file.txt").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            for _ in 0..size {
                workers.push(Worker::new(Arc::clone(&receiver)));
            }
            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.take());
            for worker in &mut self.workers {
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

    struct Worker {
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        job();
                    }
                    Err(_) => {
                        break;
                    }
                }
            });

            Worker {
                thread: Some(thread),
            }
        }
    }
}
