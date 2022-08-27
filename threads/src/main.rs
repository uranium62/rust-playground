use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    threads();
}

pub fn threads(){
    let id = thread::current().id();
    println!("main thread:{:?}", id);

    let thread1 = ping_pong();
    let thread2 = ping_pong();

    thread1.join().unwrap();
    thread2.join().unwrap();
}

pub fn ping_pong() -> JoinHandle<()> {
    thread::spawn(|| {
        let id = thread::current().id();
        for i in 1..5 {
            println!("ping:{}, thread:{:?}", i, id);
            thread::sleep(Duration::from_millis(1000));
            println!("pong:{}, thread:{:?}", i, id);
        }
    })
}
