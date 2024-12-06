# Advent of Code

Here are some benchmark results of my implementations. This is a very barebones benchmarking
solution.

All benchmarks were run on an M1 Pro MBP. Benchmarking is performed using a very barebones
benchmarking solution build into the workbench. Each solution is run for 1 second as a warm-up,
and then the solution is run for 10 seconds and timings are recorded. The tables below indicate
the average performance during that run.

## 2015

| Day | Step 1   | Step 2   |
|-----|----------|----------|
| 1   | 16.2 us  | 8.6 us   |
| 2   | 69.1 us  | 96.1 us  |
| 3   | 209.1 us | 220.5 us |
| 4   | 54.6 ms  | 1.826 s  |
| 5   | 72.5 us  | 114.5 us |
| 6   | 419.5 ms | 358.3 ms |
| 7   | 426.5 us | 877.5 us |
| 8   | 183 us   | 51.9 us  |
| 9   | 12.1 ms  | 12.1ms   |

## 2022

| Day | Step 1   | Step 2   |
|-----|----------|----------|
| 1   | 38.1 us  | 38 us    |
| 2   | 53.9 us  | 53.3 us  |
| 3   | 38.7 us  | 29.4 us  |
| 4   | 133.2 us | 132.3 us |
| 5   | 39 us    | 59.5 us  |
| 6   | 101.3 us | 361.7 us |
| 7   | 197.3 us | 196.2 us |
| 8   | 177.8 us | 234.7 us |
| 9   | 2.1 ms   | 2 ms     |
| 10  | 16 us    | 16.6 us  |
| 11  | 18.1 us  | 5 ms     |
| 12  | 94.3 ms  | 95.4 ms  |
| 13  | 381.4 us | 620.3 us |
| 14  | 4.5 ms   | 187.6 ms |
| 15  | 663.5 ms | 716.6 ms |

## 2023

**Note**: Day 12 was not implemented.

| Day | Step 1   | Step 2   |
|-----|----------|----------|
| 1   | 41 us    | 221.4 us |
| 2   | 50.4 us  | 50.7 us  |
| 3   | 5.3 ms   | 754.4 ms |
| 4   | 163.5 us | 164.3 us |
| 5   | 40.4 us  | 172.9 s  |
| 6   | 7.2 us   | 39.2 ms  |
| 7   | 307 us   | 311.5 us |
| 8   | 762.9 us | 3.2 ms   |
| 9   | 205.6 us | 205.6 us |
| 10  | 82.4 ms  | 82.5 ms  |
| 11  | 3.3 ms   | 3.3 ms   |
| 12  | n/a      | n/a      |
| 13  | 135.4 us | 134.9 us |
| 14  | 45 us    | 44.8 ms  |
| 15  | 56.3 us  | 214 us   |
| 16  | 303.7 us | 25.5 ms  |

## 2024

| Day | Step 1   | Step 2   |
|-----|----------|----------|
| 1   | 55.2 us  | 182 us   |
| 2   | 101.6 us | 185 us   |
| 3   | 121.8 us | 96 us    |
| 4   | 1.7 ms   | 759.5 us |
| 5   | 152.6 us | 367.2 us |
| 6   | 828.6 us | 1.870 s  |

# Improvements

- Common data structures:
  - Map/Grid (2022/08, 2022/14)
  - Path (2022/09)
  - Coordinates (2022/09, 2022/14)
