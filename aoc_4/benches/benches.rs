use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_4::{calculate_total_scratchers, calculate_winnings};

fn bench_total(c: &mut Criterion) {
    let fp: &str = "resources/input.txt";
    c.bench_function("total scratch cards", |b| {
        b.iter(|| calculate_total_scratchers(black_box(fp)).unwrap())
    });
}

fn bench_winnings(c: &mut Criterion) {
    let fp: &str = "resources/input.txt";
    c.bench_function("total winnings", |b| {
        b.iter(|| calculate_winnings(black_box(fp)).unwrap())
    });
}

criterion_group!(benches, bench_winnings, bench_total);

criterion_main!(benches);
