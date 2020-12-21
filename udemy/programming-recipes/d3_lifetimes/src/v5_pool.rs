use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

pub struct ThreadPool {
    n: u32,
    ch_s: Option<mpsc::Sender<Box<dyn Fn() + Send>>>,
    ch_done: mpsc::Receiver<()>,
}

impl ThreadPool {
    pub fn new(n: u32) -> ThreadPool {
        let (ch_s, ch_r) = mpsc::channel();
        let r = Arc::new(Mutex::new(ch_r));

        let (ch_done_s, ch_done_r) = mpsc::channel();

        for _ in 0..n {
            let r2 = r.clone();
            let done = ch_done_s.clone();
            std::thread::spawn(move || loop {
                let m = r2.lock().unwrap();
                let f: Box<dyn Fn() + Send> = match m.recv() {
                    Ok(f) => f,
                    Err(_) => {
                        done.send(()).ok();
                        return;
                    }
                };
                drop(m);
                f();
            });
        }

        ThreadPool {
            n,
            ch_s: Some(ch_s),
            ch_done: ch_done_r,
        }
    }

    pub fn run<F: Fn() + Send + 'static>(&self, f: F) {
        if let Some(ref s) = self.ch_s {
            s.send(Box::new(f)).unwrap();
        }
    }

    pub fn wait(mut self) {
        self.ch_s.take();
        for _ in 0..self.n {
            self.ch_done.recv().unwrap();
        }
    }
}

pub fn main() {
    let tp = ThreadPool::new(10);
    for i in 0..100 {
        tp.run(move || {
            std::thread::sleep(Duration::from_millis(200));
            println!("run = {}", i);
        })
    }

    tp.wait();

    //std::thread::sleep(Duration::from_millis(3000))
}
