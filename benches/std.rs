#![feature(test)]

extern crate crossbeam;
extern crate test;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use test::{black_box, Bencher};

/// An allocator using the std mutex instead of
/// the parking_lot one.
pub struct StdAlloc {
    counter: AtomicUsize,
    free: Mutex<Vec<usize>>,
}

impl StdAlloc {
    pub fn new() -> Self {
        StdAlloc {
            counter: AtomicUsize::new(0),
            free: Mutex::new(Vec::new()),
        }
    }

    pub fn alloc(&self) -> usize {
        self.free
            .lock()
            .unwrap()
            .pop()
            .unwrap_or_else(|| self.counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn kill(&self, id: usize) {
        self.free.lock().unwrap().push(id);
    }
}

#[bench]
fn std_alloc(b: &mut Bencher) {
    let alloc = StdAlloc::new();

    b.iter(|| alloc.alloc());
}

#[bench]
fn std_alloc_killed(b: &mut Bencher) {
    let alloc = StdAlloc::new();

    for i in 0..100_000 {
        alloc.kill(i);
    }

    b.iter(|| alloc.alloc());
}

#[bench]
fn std_kill(b: &mut Bencher) {
    let alloc = StdAlloc::new();

    b.iter(|| alloc.kill(0));
}

#[bench]
fn std_parallel_alloc(b: &mut Bencher) {
    let alloc = StdAlloc::new();

    b.iter(|| {
        crossbeam::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..50 {
                        black_box(alloc.alloc());
                    }
                });
            }
        })
    });
}
