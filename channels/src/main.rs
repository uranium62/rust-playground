use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    let command = tx1.clone();
    
    let thread1 = thread::spawn(move|| {
        for cmd in rx2 {
            println!("{}", cmd);
            tx1.send("ping").unwrap();
        }
    });
    
    let thread2 = thread::spawn(move|| {
        for cmd in rx1 { 
            println!("{}", cmd);
            tx2.send("pong").unwrap()
        }
    });

    command.send("ping").unwrap();
    
    thread1.join().unwrap();
    thread2.join().unwrap();
}
