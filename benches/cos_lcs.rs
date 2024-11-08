use cos_lcs::{cos_lcs::ClosestOffsetSumLcs, lcs_trait::Lcs};
use divan::{black_box, Bencher};
use rand::distributions::{Distribution, Uniform};

const LENGTH: usize = 1000;

#[divan::bench]
fn new(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(LENGTH).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(LENGTH).collect();
    let source = black_box(&source);
    let target = black_box(&target);
    bencher.bench_local(move || ClosestOffsetSumLcs::new(source, target));
}

#[divan::bench]
fn length(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(LENGTH).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(LENGTH).collect();
    let lcs = ClosestOffsetSumLcs::new(black_box(&source), black_box(&target));
    bencher.bench_local(move || lcs.len());
}

#[divan::bench]
fn subsequence(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(LENGTH).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(LENGTH).collect();
    let lcs = ClosestOffsetSumLcs::new(black_box(&source), black_box(&target));
    bencher.bench_local(move || lcs.subsequence());
}
