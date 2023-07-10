use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
// use rand::Rng;

fn bench_shuffle(c: &mut Criterion) {
    let index = 459;
    let binary = format!("{:?}", format!("{:08b}", 8));

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Shuffle");

    let i = &100u64;

    group.bench_with_input(
        BenchmarkId::new("Parse Binary", i.to_owned()),
        i,
        |b, _i| b.iter(|| predictive_shuffle::parse_binary(binary.as_str())),
    );

    group.bench_with_input(
        BenchmarkId::new("Shuffle 100_000", i.to_owned()),
        i,
        |b, _i| b.iter(|| predictive_shuffle::shuffle(100_000, &vec![100], index)),
    );

    group.bench_with_input(
        BenchmarkId::new("Shuffle 10_000_000", i.to_owned()),
        i,
        |b, _i| b.iter(|| predictive_shuffle::shuffle(10_000_000, &vec![100], index)),
    );

    group.finish();
}

criterion_group!(benches, bench_shuffle);
criterion_main!(benches);
