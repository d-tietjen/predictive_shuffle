use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
// use rand::Rng;

fn bench_shuffle(c: &mut Criterion) {
    let size = 1_000;
    let peers = (size as f32).log10() as usize;
    let items: Vec<usize> = (0..(10 * peers)).collect();
    let seed = b"1abc3edf".to_vec();
    let ordered_vec: Vec<usize> = (0..size).collect();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Shuffle");

    let i = &50u64;

    group.bench_with_input(BenchmarkId::new("Vec Shuffle", i.to_owned()), i, |b, _i| {
        b.iter(|| {
            let vec = predictive_shuffle::shuffle_vec(ordered_vec.clone(), &seed, size);
            for i in &items {
                let _a = vec[*i];
            }
        })
    });

    // group.bench_with_input(BenchmarkId::new("Shuffle", i.to_owned()), i, |b, _i| {
    //     b.iter(|| {
    //         for i in &items {
    //             predictive_shuffle::shuffle_prediction(*i, &seed, size);
    //         }
    //     })
    // });

    group.bench_with_input(BenchmarkId::new("Multi", i.to_owned()), i, |b, _i| {
        b.iter(|| predictive_shuffle::multi_index_shuffle_prediction(&items, &seed, size))
    });

    // group.bench_with_input(
    //     BenchmarkId::new("Shuffle 10_000", i.to_owned()),
    //     i,
    //     |b, _i| b.iter(|| predictive_shuffle::worst_case(0, &seed, 10_000)),
    // );

    // group.bench_with_input(
    //     BenchmarkId::new("Shuffle 100_000", i.to_owned()),
    //     i,
    //     |b, _i| b.iter(|| predictive_shuffle::worst_case(0, &seed, 100_000)),
    // );

    // group.bench_with_input(
    //     BenchmarkId::new("Shuffle 1_000_000", i.to_owned()),
    //     i,
    //     |b, _i| b.iter(|| predictive_shuffle::worst_case(0, &seed, 1_000_000)),
    // );

    group.finish();
}

criterion_group!(benches, bench_shuffle);
criterion_main!(benches);
