# Benchmark – Two Sets

## Objective

The objective is to divide the numbers from 1 to n into two sets having equal sum whenever such a partition exists.

Two different algorithms are compared:

1. Greedy Construction
2. Mathematical Construction

---

## Algorithm 1 – Greedy Construction

The greedy algorithm begins with the largest number and repeatedly places it into the first set whenever it does not exceed the remaining target sum. The remaining numbers are placed into the second set.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Time | O(n) |
| Extra Space | O(n) |

### Advantages

- Simple and intuitive.
- Easy to implement.
- Works efficiently for very large values of n.

### Disadvantages

- Decisions are made locally.
- Requires maintaining a remaining target value.

---

## Algorithm 2 – Mathematical Construction

This algorithm exploits the mathematical property that a valid partition exists only when the total sum is even. Depending on n mod 4, it directly constructs the two sets using a predefined pairing pattern without making greedy choices.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Time | O(n) |
| Extra Space | O(n) |

### Advantages

- Direct mathematical solution.
- No greedy decisions.
- Very elegant and deterministic.

### Disadvantages

- Requires understanding the mathematical proof behind the construction.
- Less intuitive than the greedy approach.

---

## Comparison

| Feature | Greedy | Mathematical Construction |
|---------|---------|---------------------------|
| Time Complexity | O(n) | O(n) |
| Extra Space | O(n) | O(n) |
| Strategy | Local Decisions | Direct Construction |
| Easy to Explain | Yes | Moderate |

---

## Conclusion

Both implementations solve the Two Sets problem correctly for the tested cases and produce a valid partition when one exists.

The greedy approach builds the partition by repeatedly placing the largest feasible numbers into the first set, while the constructive approach uses a direct pattern based on the value of $n$ modulo 4.

For benchmarking, it is useful to compare runtime and output size for larger values of $n$ and to verify that each partition has equal sums.