use std::{marker::PhantomData, ops::Deref, ptr::NonNull, sync::atomic::AtomicUsize};

struct ArcInner<T> {
    value: T,
    count: AtomicUsize,
}

impl<T> ArcInner<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            count: AtomicUsize::new(1),
        }
    }
}

struct Arc<T> {
    value: NonNull<ArcInner<T>>,
    phantom: PhantomData<T>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.value.as_ref().value }
    }
}

impl<T> Arc<T> {
    pub fn new(value: T) -> Self {
        let value = Box::new(ArcInner::new(value));
        Self {
            value: unsafe { NonNull::new_unchecked(Box::into_raw(value)) },
            phantom: PhantomData,
        }
    }

    pub fn get_strong_count(&self) -> usize {
        unsafe {
            self.value
                .as_ref()
                .count
                .load(std::sync::atomic::Ordering::Acquire)
        }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        // We would increase the counter of arc.
        unsafe { self.value.as_ref() }
            .count
            .fetch_add(1, std::sync::atomic::Ordering::Acquire);
        Self {
            value: self.value,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // If returned value is 1 this is the last owner to the value, & we can drop the pointer.
        if unsafe {
            self.value
                .as_ref()
                .count
                .fetch_sub(1, std::sync::atomic::Ordering::Acquire)
                == 1
        } {
            let _ = unsafe { Box::from_raw(self.value.as_ptr()) };
        }
    }
}

fn main() {
    let value = Arc::new(String::new());

    let value2 = value.clone();

    {
        let value3 = value.clone();
        assert_eq!(3, value.get_strong_count());
    }

    //eg_take::<String>(&value);

    assert_eq!(2, value.get_strong_count());
}

fn eg_take<T>(value: &T) {}
