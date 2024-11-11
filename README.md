# cos_lcs

A heuristic algorithm for approximating the Longest Common Subsequence (LCS) problem.

## Overview

**cos_lcs** stands for **Closest Offset Sum LCS**. It is a heuristic algorithm designed to approximate the Longest Common Subsequence between two sequences. While it may not consistently find the full LCS, it provides a fast approximation with linear time complexity, making it suitable for large inputs where classical algorithms become impractical.

## Algorithm Details

The algorithm implementation can be found in [src/cos_lcs.rs](src/cos_lcs.rs). Unlike the classical dynamic programming approach for LCS, which has a time complexity of O(N×M), `cos_lcs` offers a linear time approximation. Although it does not guarantee the longest possible common subsequence, it ensures that any subsequence it finds is a valid common subsequence of the input sequences.

### Time Complexity

- **Time Complexity**: O(N + M)

### Approximation Factor

In the context of LCS approximation (a maximization problem), the approximation factor of `cos_lcs` is α = 0.8. This means that the length of the subsequence found by the algorithm is at least 80% of the optimal LCS length.

## Benchmarks and Testing

You can adjust the input lengths for most tests and benchmarks in this repository. Note that input lengths over `10,000` for single tests or `3,000` for benchmarks may result in long wait times if the classical LCS algorithm is involved, as in [benches/slow_lcs.rs](benches/slow_lcs.rs).

### Running Benchmarks

Run benchmarks with:

```sh
cargo bench
```

Adjust the `LENGTH` constant in [benches/cos_lcs.rs](benches/cos_lcs.rs) and [benches/slow_lcs.rs](benches/slow_lcs.rs) to modify input lengths.

### Testing Accuracy

Test the accuracy of the algorithm by running:

```sh
cargo test test_accuracy -- --show-output
```

The input length for the tests can also be controlled via the `LENGTH` constant in the test files.

## Related Work

This project was compared to the following sources:

- [Approximation Algorithms for the LCS Problem](https://arxiv.org/abs/2111.10538)
- [Fast Approximate String Matching Algorithms](https://arxiv.org/abs/2106.08195)
- [Heuristics for the Longest Common Subsequence Problem](https://arxiv.org/abs/2105.03028)
- [Efficient Algorithms for String Comparison](https://arxiv.org/pdf/2003.07285)

## Building from Source

1. Install the stable branch of Rust using [Rustup](https://rustup.rs/).
2. Clone the repository and navigate to the project root directory.
3. Compile the program:

   ```sh
   cargo build
   ```

## License

[Todo]

## Contributing

[Todo]
