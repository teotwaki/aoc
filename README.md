# Advent of Code

Here are some benchmark results of my implementations. This is a very barebones benchmarking
solution.

All benchmarks were run on an M1 Pro MBP. Benchmarking is performed using a very barebones
benchmarking solution build into the workbench. Each solution is run for 1 second as a warm-up,
and then the solution is run for 10 seconds and timings are recorded. The tables below indicate
the average performance during that run.

## 2015

| Day                      | Step 1     | Step 2     |
|--------------------------|------------|------------|
| [1](2015/01/src/lib.rs)  | 16.2 us    | 8.6 us     |
| [2](2015/02/src/lib.rs)  | 69.1 us    | 96.1 us    |
| [3](2015/03/src/lib.rs)  | 209.1 us   | 220.5 us   |
| [4](2015/04/src/lib.rs)  | 54.593 ms  | 1.826 s    |
| [5](2015/05/src/lib.rs)  | 72.5 us    | 114.5 us   |
| [6](2015/06/src/lib.rs)  | 419.436 ms | 358.320 ms |
| [7](2015/07/src/lib.rs)  | 426.5 us   | 877.5 us   |
| [8](2015/08/src/lib.rs)  | 182.9 us   | 51.9 us    |
| [9](2015/09/src/lib.rs)  | 12.150 ms  | 12.138 ms  |
| [10](2015/10/src/lib.rs) | 16.079 ms  | 233.832 ms |
| [11](2015/11/src/lib.rs) | 261.0 us   | 11.540 ms  |
| [12](2015/12/src/lib.rs) | 113.3 us   | 202.6 us   |
| [13](2015/13/src/lib.rs) | 24.352 ms  | 242.300 ms |
| [14](2015/14/src/lib.rs) | 8.0 us     | 105.5 us   |
| [15](2015/15/src/lib.rs) | 2.446 s    | 2.436 s    |
| [16](2015/16/src/lib.rs) | 62.8 us    | 48.0 us    |

## 2022

| Day                      | Step 1     | Step 2     |
|--------------------------|------------|------------|
| [1](2022/01/src/lib.rs)  | 38.1 us    | 38.0 us    |
| [2](2022/02/src/lib.rs)  | 53.9 us    | 53.3 us    |
| [3](2022/03/src/lib.rs)  | 38.7 us    | 29.4 us    |
| [4](2022/04/src/lib.rs)  | 133.2 us   | 132.3 us   |
| [5](2022/05/src/lib.rs)  | 39.1 us    | 59.5 us    |
| [6](2022/06/src/lib.rs)  | 101.3 us   | 361.7 us   |
| [7](2022/07/src/lib.rs)  | 197.3 us   | 196.2 us   |
| [8](2022/08/src/lib.rs)  | 177.8 us   | 234.7 us   |
| [9](2022/09/src/lib.rs)  | 2.101 ms   | 2.001 ms   |
| [10](2022/10/src/lib.rs) | 16.0 us    | 16.6 us    |
| [11](2022/11/src/lib.rs) | 18.1 us    | 4.969 ms   |
| [12](2022/12/src/lib.rs) | 94.291 ms  | 95.384 ms  |
| [13](2022/13/src/lib.rs) | 381.4 us   | 620.3 us   |
| [14](2022/14/src/lib.rs) | 4.522 ms   | 187.592 ms |
| [15](2022/15/src/lib.rs) | 666.804 ms | 421.006 ms |

## 2023

**Note**: Day 12 was not implemented.

| Day                      | Step 1    | Step 2     |
|--------------------------|-----------|------------|
| [1](2023/01/src/lib.rs)  | 41.0 us   | 221.4 us   |
| [2](2023/02/src/lib.rs)  | 50.4 us   | 50.7 us    |
| [3](2023/03/src/lib.rs)  | 5.276 ms  | 754.365 ms |
| [4](2023/04/src/lib.rs)  | 163.5 us  | 164.3 us   |
| [5](2023/05/src/lib.rs)  | 39.3 us   | 23.859 s   |
| [6](2023/06/src/lib.rs)  | 7.2 us    | 39.156 ms  |
| [7](2023/07/src/lib.rs)  | 306.9 us  | 311.5 us   |
| [8](2023/08/src/lib.rs)  | 762.9 us  | 3.232 ms   |
| [9](2023/09/src/lib.rs)  | 205.6 us  | 205.6 us   |
| [10](2023/10/src/lib.rs) | 82.433 ms | 82.464 ms  |
| [11](2023/11/src/lib.rs) | 3.303 ms  | 3.299 ms   |
| [13](2023/13/src/lib.rs) | 135.4 us  | 134.9 us   |
| [14](2023/14/src/lib.rs) | 45.0 us   | 44.820 ms  |
| [15](2023/15/src/lib.rs) | 56.3 us   | 214.0 us   |
| [16](2023/16/src/lib.rs) | 303.7 us  | 25.469 ms  |

## 2024

| Day                     | Step 1    | Step 2     |
|-------------------------|-----------|------------|
| [1](2024/01/src/lib.rs) | 55.2 us   | 182.0 us   |
| [2](2024/02/src/lib.rs) | 101.6 us  | 185.0 us   |
| [3](2024/03/src/lib.rs) | 121.8 us  | 96.0 us    |
| [4](2024/04/src/lib.rs) | 1.669 ms  | 759.5 us   |
| [5](2024/05/src/lib.rs) | 151.7 us  | 242.2 us   |
| [6](2024/06/src/lib.rs) | 831.0 us  | 246.699 ms |
| [7](2024/07/src/lib.rs) | 4.255 ms  | 136.255 ms |

# Improvements

- Common data structures:
  - Map/Grid (2022/08, 2022/14)
  - Path (2022/09)
  - Coordinates (2022/09, 2022/14)
