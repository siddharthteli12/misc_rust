use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let signal1 = Arc::new(Mutex::new(false));
    let signal2 = Arc::clone(&signal1);
    // thread to write hello.
    let handler1 = thread::spawn(move || loop {
        {
            let mut signal = signal2.lock().unwrap();
            if !*signal {
                print!("Hello");
                *signal = true;
            }
        }
        thread::sleep(Duration::from_secs(1));
    });

    let handler2 = thread::spawn(move || loop {
        {
            let mut signal = signal1.lock().unwrap();
            if *signal {
                println!("World");
                *signal = false;
            }
        }
        thread::sleep(Duration::from_secs(1));
    });

    handler1.join().unwrap();
    handler2.join().unwrap();
}
