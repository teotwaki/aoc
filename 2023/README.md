The following benchmarks were run on the same laptop with an M1 Pro CPU.

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

*Note*: It seems that 2.8ms/2.9ms is the lowest time that either a process takes to start up on
MacOS, or the smallest unit that `hyperfine` is able to benchmark
