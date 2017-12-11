#![feature(test)]

extern crate crossbeam;
extern crate test;

use std::sync::atomic::{AtomicUsize, Ordering};

use crossbeam::sync::MsQueue as Queue;
use test::{black_box, Bencher};

/// An allocator using a queue instead of
/// a mutex for performance comparision.
pub struct QueueAlloc {
    counter: AtomicUsize,
    free: Queue<usize>,
}

impl QueueAlloc {
    pub fn new() -> Self {
        QueueAlloc {
            counter: AtomicUsize::new(0),
            free: Queue::new(),
        }
    }

    pub fn alloc(&self) -> usize {
        self.free
            .try_pop()
            .unwrap_or_else(|| self.counter.fetch_add(1, Ordering::Relaxed))
    }

    pub fn kill(&self, id: usize) {
        self.free.push(id);
    }
}

#[bench]
fn queue_alloc(b: &mut Bencher) {
    let alloc = QueueAlloc::new();

    b.iter(|| alloc.alloc());
}

#[bench]
fn queue_alloc_killed(b: &mut Bencher) {
    let alloc = QueueAlloc::new();

    for i in 0..100_000 {
        alloc.kill(i);
    }

    b.iter(|| alloc.alloc());
}

#[bench]
fn queue_kill(b: &mut Bencher) {
    let alloc = QueueAlloc::new();

    b.iter(|| alloc.kill(0));
}

#[bench]
fn queue_parallel_alloc(b: &mut Bencher) {
    let alloc = QueueAlloc::new();

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
