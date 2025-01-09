use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;
use std::time::Duration;
fn main() {
    let condition = Condvar::new();
    let flag = Mutex::new(false);
    thread::scope(|s| {
        // Thread 1: Print hello and notify.
        s.spawn(|| loop {
            let mut flag = flag.lock().unwrap();
            if !*flag {
                print!("Hello");
                *flag = true;
                condition.notify_one();
            }
            drop(flag);
            thread::sleep(Duration::from_secs(1));
        });

        // Thread 2: Print world and park.
        s.spawn(|| {
            let mut flag = flag.lock().unwrap();
            loop {
                if *flag {
                    println!("World");
                    *flag = false;
                }
                flag = condition.wait(flag).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
    });

    thread::sleep(Duration::from_secs(10));
}
