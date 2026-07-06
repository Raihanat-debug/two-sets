# Two Sets

This repository contains Rust implementations for the classic CSES "Two Sets" problem.

The goal is to partition the numbers from 1 to $n$ into two groups with equal sum whenever such a partition exists.

## Implementations

- Greedy approach: [src/bin/greedy.rs](src/bin/greedy.rs)
- Constructive mathematical approach: [src/bin/constructive.rs](src/bin/constructive.rs)

Both programs read an integer $n$ from standard input and print either:

- `NO` if the partition is impossible, or
- `YES` followed by a valid partition into two sets when it exists.

## Project structure

- [src/bin/greedy.rs](src/bin/greedy.rs) – greedy solver
- [src/bin/constructive.rs](src/bin/constructive.rs) – constructive solver
- [tests/greedy_tests.rs](tests/greedy_tests.rs) – regression tests for the greedy binary
- [tests/constructive_tests.rs](tests/constructive_tests.rs) – regression tests for the constructive binary
- [benchmark.md](benchmark.md) – notes comparing the two approaches

## Running the binaries

```bash
cargo run --bin greedy
cargo run --bin constructive
```

## Running tests

```bash
cargo test
```

## Notes

The mathematical constructive solution is based on the known condition that a partition exists only when the total sum is even, and it uses a direct pattern depending on $n \bmod 4$.
