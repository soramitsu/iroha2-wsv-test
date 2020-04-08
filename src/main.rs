fn main() {
    println!("Hello, world!");
    let (tx, rx) = mpsc::channel();
    let kura = Kura { tx };
    thread::spawn(move || loop {
        kura.tx.send("Test".to_string()).unwrap();
        thread::sleep(Duration::from_millis(200));
    });
    let wsv = Wsv {
        inner: Arc::new(Mutex::new(Vec::new())),
        rx: Arc::new(Mutex::new(rx)),
    };
    let mut torii = Torii { wsv };
    torii.start();
}

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

struct Torii {
    wsv: Wsv,
}

impl Torii {
    pub fn start(&mut self) {
        self.wsv.start();
        loop {
            self.wsv.read();
            thread::sleep(Duration::from_millis(100));
        }
    }
}

struct Wsv {
    inner: Arc<Mutex<Vec<String>>>,
    rx: Arc<Mutex<mpsc::Receiver<String>>>,
}

impl Wsv {
    fn start(&mut self) {
        let inner_ar = Arc::clone(&self.inner);
        let rx_ar = Arc::clone(&self.rx);
        thread::spawn(move || {
            for string in rx_ar.lock().unwrap().iter() {
                println!("Got it.");
                inner_ar.lock().unwrap().push(string);
            }
        });
    }

    fn read(&self) {
        dbg!(&self.inner.lock().unwrap());
    }
}

struct Kura {
    tx: mpsc::Sender<String>,
}
