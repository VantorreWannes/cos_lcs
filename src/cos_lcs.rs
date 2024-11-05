use std::fmt::Debug;

use crate::lcs_trait::Lcs;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct ClosestOffsetSumLcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
}

impl<'a> ClosestOffsetSumLcs<'a> {
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Self { source, target }
    }

    fn next_pair_offsets(source: &[u8], target: &[u8]) -> Option<(usize, usize)> {
        let mut first_occurrence_in_target = [usize::MAX; 256];
        for (j, &byte) in target.iter().enumerate() {
            let idx = byte as usize;
            if first_occurrence_in_target[idx] == usize::MAX {
                first_occurrence_in_target[idx] = j;
            }
        }

        let mut min_sum = usize::MAX;
        let mut result = None;

        for (i, &byte) in source.iter().enumerate() {
            let idx = byte as usize;
            if let target_idx @ 0..usize::MAX = first_occurrence_in_target[idx] {
                let sum = i + target_idx;
                if sum < min_sum {
                    min_sum = sum;
                    result = Some((i, target_idx));
                }

                if i >= min_sum {
                    break;
                }
            }
        }

        result
    }
}

impl<'a> Lcs<u8> for ClosestOffsetSumLcs<'a> {
    fn subsequence(&self) -> Vec<u8> {
        let (mut last_source_pair_index, mut last_target_pair_index) = (0, 0);
        let mut lcs: Vec<u8> = vec![];
        while let Some((source_pair_offset, target_pair_offset)) = Self::next_pair_offsets(
            &self.source[last_source_pair_index..],
            &self.target[last_target_pair_index..],
        ) {
            last_source_pair_index += source_pair_offset + 1;
            last_target_pair_index += target_pair_offset + 1;
            lcs.push(self.source[last_source_pair_index - 1]);
        }
        lcs
    }

    fn len(&self) -> usize
    where
        Self: Sized,
    {
        self.subsequence().len()
    }

    fn is_empty(&self) -> bool
    where
        Self: Sized,
    {
        self.len() == 0
    }
}

#[cfg(test)]
mod closest_offset_sum_lcs_tests {
    use super::*;

    #[test]
    fn new() {
        let source = vec![0u8; 100];
        let target = source.clone();
        _ = ClosestOffsetSumLcs::new(&source, &target);
    }

    #[test]
    fn next_pair_offsets() {
        let source = [2, 1, 0, 3];
        let target = [0, 1, 2, 3];
        let next_pair_offsets = ClosestOffsetSumLcs::<'_>::next_pair_offsets(&source, &target);
        assert_eq!(next_pair_offsets, Some((0, 2)));
    }

    #[test]
    fn subsequence() {
        let source = [2, 1, 0, 3];
        let target = [0, 1, 2, 3];
        let closest_offset_sum_lcs = ClosestOffsetSumLcs::new(&source, &target);
        assert_eq!(closest_offset_sum_lcs.subsequence(), vec![2, 3]);
    }
}
