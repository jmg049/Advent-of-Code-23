use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2::{power, sum};

fn bench_power(c: &mut Criterion) {
    let fp: &str = "resources/input.txt";
    c.bench_function("power", |b| b.iter(|| power(black_box(fp))));
}

fn bench_sum(c: &mut Criterion) {
    let fp: &str = "resources/input.txt";
    c.bench_function("sum", |b| b.iter(|| sum(black_box(fp))));
}

criterion_group!(benches, bench_power, bench_sum);

criterion_main!(benches);