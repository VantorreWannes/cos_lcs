use std::time::Duration;

use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, Throughput};
use cos_lcs::{cos_lcs::ClosestOffsetSumLcs, lcs_trait::Lcs, slow_lcs::SlowLcs};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const LENGTHS: [usize; 7] = [10, 100, 250, 500, 1000, 1500, 2000];
const ALPHABET_SIZES: [u8; 8] = [1, 2, 4, 16, 32, 64, 128, 255];

fn benchmark_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("LCS_new");
    for &alphabet_size in ALPHABET_SIZES.iter() {
        for &length in LENGTHS.iter() {
            configure_group(&mut group, length);
            let (source, target) = generate_random_data(length, alphabet_size);
            let source = black_box(source);
            let target = black_box(target);

            let id = BenchmarkId::new("ClosestOffsetSumLcs", format!("α{alphabet_size}_n{length}_m{length}"));
            group.bench_function(id, |b| {
                b.iter(|| ClosestOffsetSumLcs::new(&source,&target))
            });

            let id = BenchmarkId::new("SlowLcs", format!("α{alphabet_size}_n{length}_m{length}"));
            group.bench_function(id, |b| {
                b.iter(|| SlowLcs::new(&source,&target))
            });
        }
    }
    group.finish();
}

fn benchmark_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("LCS_length");
    for &alphabet_size in ALPHABET_SIZES.iter() {
        for &length in LENGTHS.iter() {
            configure_group(&mut group, length);
            let (source, target) = generate_random_data(length, alphabet_size);

            let lcs_cos = ClosestOffsetSumLcs::new(&source, &target);
            let lcs_slow = SlowLcs::new(&source, &target);

            let id = BenchmarkId::new("ClosestOffsetSumLcs", format!("α{alphabet_size}_n{length}_m{length}"));
            group.bench_function(id, |b| {
                b.iter(|| black_box(&lcs_cos).len())
            });

            let id = BenchmarkId::new("SlowLcs", format!("α{alphabet_size}_n{length}_m{length}"));
            group.bench_function(id, |b| {
                b.iter(|| black_box(&lcs_slow).len())
            });
        }
    }
    group.finish();
}

fn benchmark_subsequence(c: &mut Criterion) {
    let mut group = c.benchmark_group("LCS_subsequence");
    for &alphabet_size in ALPHABET_SIZES.iter() {
        for &length in LENGTHS.iter() {
            configure_group(&mut group, length);
            let (source, target) = generate_random_data(length, alphabet_size);

            let lcs_cos = ClosestOffsetSumLcs::new(&source, &target);
            let lcs_slow = SlowLcs::new(&source, &target);

            let id = BenchmarkId::new("ClosestOffsetSumLcs", format!("α{alphabet_size}_n{length}_m{length}"));
            group.bench_function(id, |b| {
                b.iter(|| black_box(&lcs_cos).subsequence())
            });

            let id = BenchmarkId::new("SlowLcs", format!("α{alphabet_size}_n{length}_m{length}"));
            group.bench_function(id, |b| {
                b.iter(|| black_box(&lcs_slow).subsequence())
            });
        }
    }
    group.finish();
}

fn configure_group(group: &mut BenchmarkGroup<'_, WallTime>, length: usize) {
    group.throughput(Throughput::Bytes(length as u64));
    if length >= 2000 {
        group.measurement_time(Duration::from_secs(400));
    }
    else if length >= 1500 {
        group.measurement_time(Duration::from_secs(300));
    }
    else if length >= 1000 {
        group.measurement_time(Duration::from_secs(200));
    }
    else if length >= 500 {
        group.measurement_time(Duration::from_secs(100));
    }
    else if length >= 100 {
        group.measurement_time(Duration::from_secs(50));
    }
    else {
        group.measurement_time(Duration::from_secs(10));
    }
}

fn generate_random_data(length: usize, alphabet_size: u8) -> (Vec<u8>, Vec<u8>) {
    let mut rng = thread_rng();
    let die = Uniform::from(0..alphabet_size);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(length).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(length).collect();
    (source, target)
}

criterion_group!(
    benches,
    benchmark_new,
    benchmark_length,
    benchmark_subsequence
);
criterion_main!(benches);
