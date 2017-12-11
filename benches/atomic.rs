#![feature(test)]

extern crate test;

use std::sync::atomic::{AtomicUsize, Ordering};
use test::Bencher;

#[bench]
fn fetch_add(b: &mut Bencher) {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let nr = AtomicUsize::new(0);

    b.iter(|| nr.fetch_add(1, Ordering::Relaxed));
}
