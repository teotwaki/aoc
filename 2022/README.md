The following benchmarks were run on the same laptop with an M1 Pro CPU.

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

*Note*: It seems that 2.8ms/2.9ms is the lowest time that either a process takes to start up on
MacOS, or the smallest unit that `hyperfine` is able to benchmark
