use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use cos_lcs::{cos_lcs::ClosestOffsetSumLcs, lcs_trait::Lcs, slow_lcs::SlowLcs};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const LENGTHS: [usize; 7] = [10, 50, 100, 500, 1000, 1500, 2000];
const ALPHABET_SIZES: [u8; 5] = [2, 4, 16, 64, 255];

/// Helper function to generate random source and target data of given length and alphabet size
fn generate_random_data(length: usize, alphabet_size: u8) -> (Vec<u8>, Vec<u8>) {
    let mut rng = thread_rng();
    let die = Uniform::from(0..=alphabet_size);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(length).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(length).collect();
    (source, target)
}

fn benchmark_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("LCS_new");
    for &alphabet_size in ALPHABET_SIZES.iter() {
        for &length in LENGTHS.iter() {
            let (source, target) = generate_random_data(length, alphabet_size);
            let source = black_box(source);
            let target = black_box(target);

            let id = BenchmarkId::new(format!("ClosestOffsetSumLcs_α{}", alphabet_size), length);
            group.bench_function(id, |b| {
                b.iter(|| ClosestOffsetSumLcs::new(&source, &target))
            });

            let id = BenchmarkId::new(format!("SlowLcs_α{}", alphabet_size), length);
            group.bench_function(id, |b| {
                b.iter(|| SlowLcs::new(&source, &target))
            });
        }
    }
    group.finish();
}

fn benchmark_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("LCS_length");
    for &alphabet_size in ALPHABET_SIZES.iter() {
        for &length in LENGTHS.iter() {
            let (source, target) = generate_random_data(length, alphabet_size);

            let lcs_cos = ClosestOffsetSumLcs::new(&source, &target);
            let lcs_slow = SlowLcs::new(&source, &target);

            let id = BenchmarkId::new(format!("ClosestOffsetSumLcs_α{}", alphabet_size), length);
            group.bench_function(id, |b| {
                b.iter(|| black_box(&lcs_cos).len())
            });

            let id = BenchmarkId::new(format!("SlowLcs_α{}", alphabet_size), length);
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
            let (source, target) = generate_random_data(length, alphabet_size);

            let lcs_cos = ClosestOffsetSumLcs::new(&source, &target);
            let lcs_slow = SlowLcs::new(&source, &target);

            let id = BenchmarkId::new(format!("ClosestOffsetSumLcs_α{}", alphabet_size), length);
            group.bench_function(id, |b| {
                b.iter(|| black_box(&lcs_cos).subsequence())
            });

            let id = BenchmarkId::new(format!("SlowLcs_α{}", alphabet_size), length);
            group.bench_function(id, |b| {
                b.iter(|| black_box(&lcs_slow).subsequence())
            });
        }
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_new,
    benchmark_length,
    benchmark_subsequence
);
criterion_main!(benches);
