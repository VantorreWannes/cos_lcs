// Property of Wannes Vantorre. DO NOT distribute.
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
        let slow_lcs_len = SlowLcs::new(&source, &target).len();
        let cos_lcs_len = ClosestOffsetSumLcs::new(&source, &target).len();
        println!("SlowLcs LCS length: {}", slow_lcs_len);
        println!("ClosestOffsetSum LCS length {}", cos_lcs_len);
        println!(
            "{} is {}% percent of {}",
            cos_lcs_len,
            (cos_lcs_len as f64 / slow_lcs_len as f64 * 100.0).round(),
            slow_lcs_len
        );
    }
}
