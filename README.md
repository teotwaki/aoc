# Advent of Code

Here are some benchmark results of my implementations. Here's what's covered by the timing:

- Spawning the binary
- Binary loads `input.txt`
- Parsing of the data
- Execution of the algorithm

All benchmarks were run on an M1 Pro MBP. Benchmarking is performed with [hyperfine][hyperfine-gh],
using the `-N` and `--warmup` options. Depending on the speed of the binary, the warmup can be
anywhere between 10 and 500.

[hyperfine-gh]: https://github.com/sharkdp/hyperfine

**Please note**: Spawning process on MacOS is *slow*. A simple binary that does nothing takes
roughly 2.5 ms to be spawned, execute and shut down. So when a benchmark below shows "2.9 ms", that
actually means the process loaded the input, parsed the data and executed the algorithm in 400
usec.

## 2022

| Day | Step 1  | Step 2   |
|-----|---------|----------|
| 1   | 2.9ms   | 2.9ms    |
| 2   | 2.9ms   | 2.9ms    |
| 3   | 2.9ms   | 2.9ms    |
| 4   | 3.0ms   | 3.0ms    |
| 5   | 2.9ms   | 2.9ms    |
| 6   | 3.0ms   | 3.8ms    |
| 7   | 3.2ms   | 3.2ms    |
| 8   | 3.1ms   | 3.2ms    |
| 9   | 5.0ms   | 5.0ms    |
| 10  | 2.9ms   | 2.9ms    |
| 11  | 2.9ms   | 11.3ms   |
| 12  | 97.4ms  | 98.5ms   |
| 13  | 3.3ms   | 3.6ms    |
| 14  | 7.5ms   | 192.0ms  |
| 15  | 650.1ms | 694.7ms  |

## 2023

**Note**: Day 12 was not implemented.

| Day | Step 1 | Step 2   |
|-----|--------|----------|
| 1   | 2.9ms  | 3.2ms    |
| 2   | 2.9ms  | 2.9ms    |
| 3   | 8.4ms  | 761.9ms  |
| 4   | 3.0ms  | 3.0ms    |
| 5   | 2.9ms  | 173.501s |
| 6   | 2.7ms  | 51.4ms   |
| 7   | 3.2ms  | 3.1ms    |
| 8   | 3.7ms  | 6.1ms    |
| 9   | 3.1ms  | 3.1ms    |
| 10  | 85.6ms | 86.3ms   |
| 11  | 6.3ms  | 6.2ms    |
| 12  | n/a    | n/a      |
| 13  | 2.9ms  | 2.9ms    |
| 14  | 2.8ms  | 48.7ms   |
| 15  | 2.8ms  | 3.1ms    |
| 15  | 3.2ms  | 29.1ms   |

## 2024

| Day | Step 1 | Step 2 |
|-----|--------|--------|
| 1   | 2.8ms  | 2.9ms  |
| 2   | 2.9ms  | 3.0ms  |
