The following benchmarks were run on the same laptop with an M1 Pro CPU.

| Day | Step 1 | Step 2 |
|-----|--------|--------|
| 1   | 2.8ms  | 2.9ms  |
| 2   | 2.9ms  | 3.0ms  |

*Note*: It seems that 2.8ms/2.9ms is the lowest time that either a process takes to start up on
MacOS, or the smallest unit that `hyperfine` is able to benchmark
