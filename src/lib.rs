pub mod cos_lcs;
pub mod lcs_trait;
pub mod slow_lcs;

#[cfg(test)]
mod tests {
    use rand::{distributions::Uniform, prelude::Distribution};

    use crate::{cos_lcs::ClosestOffsetSumLcs, lcs_trait::Lcs, slow_lcs::SlowLcs};

    const LENGTH: usize = 2500;

    #[test]
    fn test_accuracy() {
        let mut rng = rand::thread_rng();
        let die: Uniform<u8> = Uniform::from(0..=255);
        let source: Vec<u8> = die.sample_iter(&mut rng).take(LENGTH).collect();
        let target: Vec<u8> = die.sample_iter(&mut rng).take(LENGTH).collect();
        println!(
            "SlowLcs LCS length: {}",
            SlowLcs::new(&source, &target).len()
        );
        println!(
            "ClosestOffsetSum LCS length {}",
            ClosestOffsetSumLcs::new(&source, &target).len()
        );
    }
}
