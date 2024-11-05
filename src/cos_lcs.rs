use std::fmt::Debug;

use crate::lcs_trait::Lcs;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct ClosestOffsetSumLcs<'a>
{
    source: &'a [u8],
    target: &'a [u8],
}

impl<'a> ClosestOffsetSumLcs<'a>
{
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Self { source, target }
    }

    fn next_pair_offsets(&self) -> Option<(usize, usize)> {
        let mut index1 = [usize::MAX; u8::MAX as usize + 1];
        let mut index2 = [usize::MAX; u8::MAX as usize + 1];

        // Build index1
        for (i, &v) in self.source.iter().enumerate() {
            let v = v as usize;
            if index1[v] == usize::MAX || i < index1[v] {
                index1[v] = i;
            }
        }

        // Build index2
        for (j, &v) in self.target.iter().enumerate() {
            let v = v as usize;
            if index2[v] == usize::MAX || j < index2[v] {
                index2[v] = j;
            }
        }

        let mut min_sum = usize::MAX;
        let mut result = None;

        // Find the minimal index sum where the byte values are equal
        for v in 0..256 {
            if index1[v] != usize::MAX && index2[v] != usize::MAX {
                let sum = index1[v] + index2[v];
                if sum < min_sum {
                    min_sum = sum;
                    result = Some((index1[v], index2[v]));
                }
            }
        }

        result
    }
}

impl<'a> Lcs<u8> for ClosestOffsetSumLcs<'a>
{
    fn subsequence(&self) -> Vec<u8> {
        todo!();
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
}
