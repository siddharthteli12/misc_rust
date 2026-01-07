use std::{
    cell::UnsafeCell,
    hint,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, fence},
};

struct Mutex<T> {
    value: UnsafeCell<T>,
    lock: AtomicBool,
}

struct MutexGuard<'a, T> {
    ref_ptr: &'a Mutex<T>,
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        fence(std::sync::atomic::Ordering::Acquire);
        self.ref_ptr
            .lock
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ref_ptr.value.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ref_ptr.value.get() }
    }
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        // If lock is true keep waiting forever.

        while self
            .lock
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            hint::spin_loop();
        }
        MutexGuard { ref_ptr: &self }
    }

    pub fn is_locked(&self) -> bool {
        fence(std::sync::atomic::Ordering::Acquire);
        self.lock.load(std::sync::atomic::Ordering::Relaxed)
    }
}

fn main() {
    let value = Mutex::new(String::new());

    {
        let mut lock = value.lock();
        *lock = String::from("Hello");
        assert_eq!(true, value.is_locked());
    }

    assert_eq!(false, value.is_locked());
    assert_eq!("Hello", &*value.lock());
}
