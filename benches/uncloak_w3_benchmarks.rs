use std::collections::HashSet;

use brykto::hasher::sha512_n;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn test_sha512_n_finds_match() {
    let mut first: u64 = 1;
    let target_bytes: Vec<u8> = vec![0x3D, 0x4B];

    loop {
        let matching_hash = sha512_n(&first.to_be_bytes(), 2);
        if matching_hash == target_bytes {
            break;
        }
        first += 1;
    }
}

fn test_sha512_n_finds_collision(number_of_bytes: usize) {
    let mut collisions = HashSet::new();
    let mut next: u64 = 1;
    let mut is_unique;
    loop {
        let matching_hash = sha512_n(&next.to_be_bytes(), number_of_bytes);
        is_unique = collisions.insert(matching_hash.clone());
        if !is_unique {
            break;
        }
        next += 1;
    }
    assert!(!is_unique);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sha512n finds match", |b| {
        b.iter(|| test_sha512_n_finds_match())
    });

    c.bench_function("sha512n finds collision (2)", |b| {
        b.iter(|| test_sha512_n_finds_collision(black_box(2)))
    });

    c.bench_function("sha512n finds collision (4)", |b| {
        b.iter(|| test_sha512_n_finds_collision(black_box(4)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
