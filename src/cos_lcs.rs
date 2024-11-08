// Property of Wannes Vantorre. DO NOT distribute.
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

    /// Finds the next pair of indices `(i, j)` such that `source[i] == target[j]`,
    /// minimizing the sum `i + j`.
    ///
    /// This method searches for the next matching pair of elements between the `source` and `target` slices,
    /// starting from the beginning of the slices. It returns the pair of offsets `(i, j)` where the sum `i + j`
    /// is minimized, effectively finding the earliest possible match in terms of combined indices.
    ///
    /// # Arguments
    ///
    /// * `source` - A slice of bytes representing the source sequence.
    /// * `target` - A slice of bytes representing the target sequence.
    ///
    /// # Returns
    ///
    /// * `Option<(usize, usize)>` - Returns `Some((i, j))` if a matching pair is found,
    ///   where `source[i] == target[j]` and the sum `i + j` is minimized.
    ///   Returns `None` if no such pair exists.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = [2, 1, 0, 3];
    /// let target = [0, 1, 2, 3];
    /// let next_pair = ClosestOffsetSumLcs::next_pair_offsets(&source, &target);
    /// assert_eq!(next_pair, Some((0, 2)));
    /// ```
    ///
    /// # Notes
    ///
    /// This method is utilized by the `subsequence` method to iteratively find matching elements
    /// between `source` and `target` sequences, constructing the Longest Common Subsequence (LCS)
    /// based on the Closest Offset Sum strategy.
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

    /// Computes the Longest Common Subsequence (LCS) between the `source` and `target` sequences
    /// using the Closest Offset Sum strategy.
    ///
    /// This method constructs the LCS by iteratively finding matching elements that minimize the sum
    /// of their indices in the `source` and `target` sequences. It ensures that the subsequence is built
    /// from left to right, respecting the order of elements in both sequences.
    ///
    /// # Returns
    ///
    /// * `Vec<u8>` - A vector containing the LCS of `source` and `target`.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = [2, 1, 0, 3];
    /// let target = [0, 1, 2, 3];
    /// let lcs_calculator = ClosestOffsetSumLcs::new(&source, &target);
    /// let lcs = lcs_calculator.subsequence();
    /// assert_eq!(lcs, vec![2, 3]);
    /// ```
    ///
    /// # Notes
    ///
    /// The `subsequence` method uses the `next_pair_offsets` method to find the next matching pair
    /// of elements between `source` and `target`, ensuring that each subsequent pair is found
    /// after the previous one to maintain the correct order in the subsequence.
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
