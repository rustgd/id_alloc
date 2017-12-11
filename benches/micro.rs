#![feature(test)]

extern crate crossbeam;
extern crate id_alloc;
extern crate test;

use id_alloc::Allocator;
use test::{black_box, Bencher};

#[bench]
fn alloc(b: &mut Bencher) {
    let alloc = Allocator::new();

    b.iter(|| alloc.alloc());
}

#[bench]
fn alloc_killed(b: &mut Bencher) {
    let alloc = Allocator::new();

    for i in 0..100_000 {
        alloc.kill(i);
    }

    b.iter(|| alloc.alloc());
}

#[bench]
fn kill(b: &mut Bencher) {
    let alloc = Allocator::new();

    b.iter(|| alloc.kill(0));
}

#[bench]
fn parallel_alloc(b: &mut Bencher) {
    let alloc = Allocator::new();

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
