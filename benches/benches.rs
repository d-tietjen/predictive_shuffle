use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use predictive_shuffle::Shuffle;

fn bench_fastrand(c: &mut Criterion) {
    let size = 100_000;
    let vec: Vec<usize> = (0..size).collect();
    let positions: Vec<usize> = (0..10).collect();
    let seed = b"love item".to_vec();
    let batch = 10;

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Shuffle");

    let i = &100u64;

    group.bench_with_input(
        BenchmarkId::new("Modern Shuffle", i.to_owned()),
        i,
        |b, _i| b.iter(|| vec.clone().modern_shuffle()),
    );

    group.bench_with_input(
        BenchmarkId::new("Modern Shuffle w/ Seed", i.to_owned()),
        i,
        |b, _i| b.iter(|| vec.clone().modern_shuffle_from_seed(seed.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("Predictive Shuffle", i.to_owned()),
        i,
        |b, _i| b.iter(|| vec.clone().predictive_shuffle(positions.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("Predictive Shuffle w/ Seed", i.to_owned()),
        i,
        |b, _i| {
            b.iter(|| {
                vec.clone()
                    .predictive_shuffle_from_seed(positions.clone(), seed.clone())
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Batch Predictive Shuffle", i.to_owned()),
        i,
        |b, _i| {
            b.iter(|| {
                vec.clone()
                    .batch_predictive_shuffle(batch, positions.clone())
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Batch Predictive Shuffle w/ Seed", i.to_owned()),
        i,
        |b, _i| {
            b.iter(|| {
                vec.clone().batch_predictive_shuffle_from_seed(
                    batch,
                    positions.clone(),
                    seed.clone(),
                )
            })
        },
    );

    group.finish();
}

fn bench_chacha(c: &mut Criterion) {
    let size = 100_000;
    let vec: Vec<usize> = (0..size).collect();
    let positions: Vec<usize> = (0..10).collect();
    let seed = b"love item".to_vec();
    let batch = 10;

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Shuffle");

    let i = &100u64;

    group.bench_with_input(
        BenchmarkId::new("Crypto Modern Shuffle", i.to_owned()),
        i,
        |b, _i| b.iter(|| vec.clone().crypto_modern_shuffle()),
    );

    group.bench_with_input(
        BenchmarkId::new("Crypto Modern Shuffle w/ Seed", i.to_owned()),
        i,
        |b, _i| b.iter(|| vec.clone().crypto_modern_shuffle_from_seed(seed.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("Crypto Predictive Shuffle", i.to_owned()),
        i,
        |b, _i| b.iter(|| vec.clone().crypto_predictive_shuffle(positions.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("Crypto Predictive Shuffle w/ Seed", i.to_owned()),
        i,
        |b, _i| {
            b.iter(|| {
                vec.clone()
                    .crypto_predictive_shuffle_from_seed(positions.clone(), seed.clone())
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Crypto Batch Predictive Shuffle", i.to_owned()),
        i,
        |b, _i| {
            b.iter(|| {
                vec.clone()
                    .crypto_batch_predictive_shuffle(batch, positions.clone())
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Crypto Batch Predictive Shuffle w/ Seed", i.to_owned()),
        i,
        |b, _i| {
            b.iter(|| {
                vec.clone().crypto_batch_predictive_shuffle_from_seed(
                    batch,
                    positions.clone(),
                    seed.clone(),
                )
            })
        },
    );

    group.finish();
}

criterion_group!(benches, bench_fastrand, bench_chacha);
criterion_main!(benches);
