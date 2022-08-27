use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = Vec::with_capacity(10);
    
    for _ in 0..10 {
        let counter = counter.clone();
        let handler = thread::spawn(move || {
            for _ in 0..10 {
                *(counter.lock().unwrap()) += 1
            }
        });
        handlers.push(handler);
    }

    for handler in handlers.into_iter(){
        handler.join().unwrap();
    }
    
    println!("{}", counter.lock().unwrap());
}
