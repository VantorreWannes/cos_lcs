// Property of Wannes Vantorre. DO NOT distribute.
pub mod cos_lcs;
pub mod lcs_trait;
pub mod slow_lcs;
pub mod stats;

#[cfg(test)]
mod tests {
    use crate::{
        cos_lcs::ClosestOffsetSumLcs,
        lcs_trait::Lcs,
        slow_lcs::SlowLcs,
        stats::{generate_random_data, serialize_stats, ComparedLcsStats},
    };

    const LENGTHS: [usize; 10] = [10, 50, 100, 500, 1000, 1500, 2000, 2500, 3000, 3500];
    const ALPHABET_SIZES: [u8; 8] = [0, 2, 4, 16, 32, 64, 128, 255];

    #[test]
    fn save_accuracy() {
        let stats = lcs_stats(&LENGTHS, &ALPHABET_SIZES);
        let mut file = std::fs::File::create("target/accuracy.csv").unwrap();
        serialize_stats(&stats, &mut file).unwrap();
    }

    fn lcs_stats(lengths: &[usize], alphabet_sizes: &[u8]) -> Vec<ComparedLcsStats> {
        let mut all_lcs_stats = vec![];
        for &input_size in lengths {
            for &alphabet_size in alphabet_sizes {
                let (source, target) = generate_random_data(input_size, alphabet_size);
                let cos_lcs = ClosestOffsetSumLcs::new(&source, &target);
                let slow_lcs = SlowLcs::new(&source, &target);
                let compared_lcs_stats =
                    ComparedLcsStats::new(input_size, alphabet_size, cos_lcs.len(), slow_lcs.len());
                all_lcs_stats.push(compared_lcs_stats);
            }
        }
        all_lcs_stats
    }
}
