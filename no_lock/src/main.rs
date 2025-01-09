use std::sync::Arc;
use std::time::Duration;
use std::{cell::UnsafeCell, collections::BTreeSet, sync::Mutex, thread};
use rand::Rng;

/// `Note` - Realising this is a garbage code even though rust doesn't have garbage collector.
// Trying some weird pattern to make data thread safe.
#[derive(Debug)]
struct Unlock {
    // Thread safe value or not?
    value: UnsafeCell<BTreeSet<String>>,
    // Mutex to make sure its thread safe?
    lock: Mutex<()>,
}

unsafe impl Send for Unlock {}
unsafe impl Sync for Unlock {}

impl Unlock {
    pub fn new() -> Self {
        Self {
            value: UnsafeCell::new(BTreeSet::new()),
            lock: Mutex::new(()),
        }
    }
}

fn main() {
    let mut value = Arc::new(Unlock::new());
    println!("Working here");
    for i in 0..100 {
        let value = value.clone();
        thread::spawn(move || loop {
            let lock = value.lock.lock().unwrap();
            let pointer = value.value.get();
            let mut rng = rand::thread_rng();
            let random_number: i32 = rng.gen_range(1..=1000000);
            let random_number1: i32 = rng.gen_range(1..=1000000);
            unsafe {
                (*pointer).insert(format!("{} {} {}", i, random_number, random_number1));
            }
            drop(lock);
        });
    }
    // Never exiting loop.
    loop {
        thread::sleep(Duration::from_secs(10));
        let pointer = value.value.get();
        let value = unsafe {
            (*pointer).clone()
        };
            println!("Value {:?}", value.len());
    }
}
