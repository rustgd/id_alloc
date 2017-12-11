extern crate parking_lot;

use std::sync::atomic::{AtomicUsize, Ordering};

use parking_lot::Mutex;

pub struct Allocator {
    counter: AtomicUsize,
    free: Mutex<Vec<usize>>,
}

impl Allocator {
    pub fn new() -> Self {
        Allocator {
            counter: AtomicUsize::new(0),
            free: Mutex::new(Vec::new()),
        }
    }

    pub fn alloc(&self) -> usize {
        self.free
            .lock()
            .pop()
            .unwrap_or_else(|| self.counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn kill(&self, id: usize) {
        self.free.lock().push(id);
    }
}
