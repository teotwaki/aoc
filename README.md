# Advent of Code

Here are some benchmark results of my implementations. This is a very barebones benchmarking
solution.

All benchmarks were run on an M1 Pro MBP. Benchmarking is performed using a very barebones
benchmarking solution built into the workbench. Each solution is run for 1 second as a warm-up,
and then the solution is run for 10 seconds and timings are recorded. The tables below indicate
the average performance during that run.

## 2015

| Day | Title                                  | Step 1     | Step 2     | Links                                                                            |
|-----|----------------------------------------|------------|------------|----------------------------------------------------------------------------------|
| 1   | Not Quite Lisp                         | 16.2 us    | 8.6 us     | [problem](https://adventofcode.com/2015/day/1) / [solution](2015/01/src/lib.rs)  |
| 2   | I Was Told There Would Be No Math      | 69.1 us    | 96.1 us    | [problem](https://adventofcode.com/2015/day/2) / [solution](2015/02/src/lib.rs)  |
| 3   | Perfectly Spherical Houses in a Vacuum | 209.1 us   | 220.5 us   | [problem](https://adventofcode.com/2015/day/3) / [solution](2015/03/src/lib.rs)  |
| 4   | The Ideal Stocking Stuffer             | 54.593 ms  | 1.826 s    | [problem](https://adventofcode.com/2015/day/4) / [solution](2015/04/src/lib.rs)  |
| 5   | Doesn't He Have Intern-Elves For This? | 72.5 us    | 114.5 us   | [problem](https://adventofcode.com/2015/day/5) / [solution](2015/05/src/lib.rs)  |
| 6   | Probably a Fire Hazard                 | 419.436 ms | 358.320 ms | [problem](https://adventofcode.com/2015/day/6) / [solution](2015/06/src/lib.rs)  |
| 7   | Some Assembly Required                 | 200.3 us   | 426.5 us   | [problem](https://adventofcode.com/2015/day/7) / [solution](2015/07/src/lib.rs)  |
| 8   | Matchsticks                            | 182.9 us   | 51.9 us    | [problem](https://adventofcode.com/2015/day/8) / [solution](2015/08/src/lib.rs)  |
| 9   | All in a Single Night                  | 5.038 ms   | 5.042 ms   | [problem](https://adventofcode.com/2015/day/9) / [solution](2015/09/src/lib.rs)  |
| 10  | Elves Look, Elves Say                  | 16.079 ms  | 233.832 ms | [problem](https://adventofcode.com/2015/day/10) / [solution](2015/10/src/lib.rs) |
| 11  | Corporate Policy                       | 261.0 us   | 11.540 ms  | [problem](https://adventofcode.com/2015/day/11) / [solution](2015/11/src/lib.rs) |
| 12  | JSAbacusFramework.io                   | 113.3 us   | 202.6 us   | [problem](https://adventofcode.com/2015/day/12) / [solution](2015/12/src/lib.rs) |
| 13  | Knights of the Dinner Table            | 8.977 ms   | 90.641 ms  | [problem](https://adventofcode.com/2015/day/13) / [solution](2015/13/src/lib.rs) |
| 14  | Reindeer Olympics                      | 8.0 us     | 105.5 us   | [problem](https://adventofcode.com/2015/day/14) / [solution](2015/14/src/lib.rs) |
| 15  | Science for Hungry People              | 2.446 s    | 2.436 s    | [problem](https://adventofcode.com/2015/day/15) / [solution](2015/15/src/lib.rs) |
| 16  | Aunt Sue                               | 58.3 us    | 44.4 us    | [problem](https://adventofcode.com/2015/day/16) / [solution](2015/16/src/lib.rs) |
| 17  | No Such Thing as Too Much              | 32.990 ms  | 34.028 ms  | [problem](https://adventofcode.com/2015/day/17) / [solution](2015/17/src/lib.rs) |
| 18  | Like a GIF For Your Yard               | 137.584 ms | 136.544 ms | [problem](https://adventofcode.com/2015/day/18) / [solution](2015/18/src/lib.rs) |
| 19  | Medicine for Rudolph                   | 3.455 ms   | 8.8 us     | [problem](https://adventofcode.com/2015/day/19) / [solution](2015/19/src/lib.rs) |
| 20  | Infinite Elves and Infinite Houses     | 45.557 ms  | 3.589 ms   | [problem](https://adventofcode.com/2015/day/20) / [solution](2015/20/src/lib.rs) |
| 21  | RPG Simulator 20XX                     | 25.6 us    | 25.5 us    | [problem](https://adventofcode.com/2015/day/21) / [solution](2015/21/src/lib.rs) |
| 22  | Wizard Simulator 20XX                  | 79.809 ms  | 501.886 ms | [problem](https://adventofcode.com/2015/day/22) / [solution](2015/22/src/lib.rs) |

## 2022

| Day | Title                   | Step 1     | Step 2     | Links                                                                            |
|-----|-------------------------|------------|------------|----------------------------------------------------------------------------------|
| 1   | Calorie Counting        | 38.1 us    | 38.0 us    | [problem](https://adventofcode.com/2022/day/1) / [solution](2022/01/src/lib.rs)  |
| 2   | Rock Paper Scissors     | 53.9 us    | 53.3 us    | [problem](https://adventofcode.com/2022/day/2) / [solution](2022/02/src/lib.rs)  |
| 3   | Rucksack Reorganization | 38.7 us    | 29.4 us    | [problem](https://adventofcode.com/2022/day/3) / [solution](2022/03/src/lib.rs)  |
| 4   | Camp Cleanup            | 133.2 us   | 132.3 us   | [problem](https://adventofcode.com/2022/day/4) / [solution](2022/04/src/lib.rs)  |
| 5   | Supply Stacks           | 39.1 us    | 59.5 us    | [problem](https://adventofcode.com/2022/day/5) / [solution](2022/05/src/lib.rs)  |
| 6   | Tuning Trouble          | 76.9 us    | 292.7 us   | [problem](https://adventofcode.com/2022/day/6) / [solution](2022/06/src/lib.rs)  |
| 7   | No Space Left On Device | 175.6 us   | 175.9 us   | [problem](https://adventofcode.com/2022/day/7) / [solution](2022/07/src/lib.rs)  |
| 8   | Treetop Tree House      | 177.8 us   | 234.7 us   | [problem](https://adventofcode.com/2022/day/8) / [solution](2022/08/src/lib.rs)  |
| 9   | Rope Bridge             | 1.282 ms   | 1.279 ms   | [problem](https://adventofcode.com/2022/day/9) / [solution](2022/09/src/lib.rs)  |
| 10  | Cathode-Ray Tube        | 16.0 us    | 16.6 us    | [problem](https://adventofcode.com/2022/day/10) / [solution](2022/10/src/lib.rs) |
| 11  | Monkey in the Middle    | 18.1 us    | 4.969 ms   | [problem](https://adventofcode.com/2022/day/11) / [solution](2022/11/src/lib.rs) |
| 12  | Hill Climbing Algorithm | 19.051 ms  | 19.723 ms  | [problem](https://adventofcode.com/2022/day/12) / [solution](2022/12/src/lib.rs) |
| 13  | Distress Signal         | 381.4 us   | 620.3 us   | [problem](https://adventofcode.com/2022/day/13) / [solution](2022/13/src/lib.rs) |
| 14  | Regolith Reservoir      | 899.4 us   | 29.490 ms  | [problem](https://adventofcode.com/2022/day/14) / [solution](2022/14/src/lib.rs) |
| 15  | Beacon Exclusion Zone   | 666.804 ms | 421.006 ms | [problem](https://adventofcode.com/2022/day/15) / [solution](2022/15/src/lib.rs) |

## 2023

**Note**: Day 12 is not implemented.

| Day | Title                           | Step 1    | Step 2     | Links                                                                            |
|-----|---------------------------------|-----------|------------|----------------------------------------------------------------------------------|
| 1   | Trebuchet?!                     | 41.0 us   | 221.4 us   | [problem](https://adventofcode.com/2023/day/1) / [solution](2023/01/src/lib.rs)  |
| 2   | Cube Conundrum                  | 50.4 us   | 50.7 us    | [problem](https://adventofcode.com/2023/day/2) / [solution](2023/02/src/lib.rs)  |
| 3   | Gear Ratios                     | 5.276 ms  | 754.365 ms | [problem](https://adventofcode.com/2023/day/3) / [solution](2023/03/src/lib.rs)  |
| 4   | Scratchcards                    | 163.5 us  | 164.3 us   | [problem](https://adventofcode.com/2023/day/4) / [solution](2023/04/src/lib.rs)  |
| 5   | If You Give A Seed A Fertilizer | 39.3 us   | 23.859 s   | [problem](https://adventofcode.com/2023/day/5) / [solution](2023/05/src/lib.rs)  |
| 6   | Wait For It                     | 7.2 us    | 39.156 ms  | [problem](https://adventofcode.com/2023/day/6) / [solution](2023/06/src/lib.rs)  |
| 7   | Camel Cards                     | 226.8 us  | 227.4 us   | [problem](https://adventofcode.com/2023/day/7) / [solution](2023/07/src/lib.rs)  |
| 8   | Haunted Wasteland               | 428.3 us  | 1.778 ms   | [problem](https://adventofcode.com/2023/day/8) / [solution](2023/08/src/lib.rs)  |
| 9   | Mirage Maintenance              | 205.6 us  | 205.6 us   | [problem](https://adventofcode.com/2023/day/9) / [solution](2023/09/src/lib.rs)  |
| 10  | Pipe Maze                       | 82.433 ms | 82.464 ms  | [problem](https://adventofcode.com/2023/day/10) / [solution](2023/10/src/lib.rs) |
| 11  | Cosmic Expansion                | 3.303 ms  | 3.299 ms   | [problem](https://adventofcode.com/2023/day/11) / [solution](2023/11/src/lib.rs) |
| 13  | Point of Incidence              | 135.4 us  | 134.9 us   | [problem](https://adventofcode.com/2023/day/13) / [solution](2023/13/src/lib.rs) |
| 14  | Parabolic Reflector Dish        | 37.2 us   | 36.229 ms  | [problem](https://adventofcode.com/2023/day/14) / [solution](2023/14/src/lib.rs) |
| 15  | Lens Library                    | 56.3 us   | 214.0 us   | [problem](https://adventofcode.com/2023/day/15) / [solution](2023/15/src/lib.rs) |
| 16  | The Floor Will Be Lava          | 303.7 us  | 25.469 ms  | [problem](https://adventofcode.com/2023/day/16) / [solution](2023/16/src/lib.rs) |


## 2024

| Day | Title                  | Step 1    | Step 2     | Links                                                                            |
|-----|------------------------|-----------|------------|----------------------------------------------------------------------------------|
| 1   | Historian Hysteria     | 55.2 us   | 182.0 us   | [problem](https://adventofcode.com/2024/day/1) / [solution](2024/01/src/lib.rs)  |
| 2   | Red-Nosed Reports      | 101.6 us  | 185.0 us   | [problem](https://adventofcode.com/2024/day/2) / [solution](2024/02/src/lib.rs)  |
| 3   | Mull It Over           | 121.8 us  | 96.0 us    | [problem](https://adventofcode.com/2024/day/3) / [solution](2024/03/src/lib.rs)  |
| 4   | Ceres Search           | 1.669 ms  | 759.5 us   | [problem](https://adventofcode.com/2024/day/4) / [solution](2024/04/src/lib.rs)  |
| 5   | Print Queue            | 136.5 us  | 234.9 us   | [problem](https://adventofcode.com/2024/day/5) / [solution](2024/05/src/lib.rs)  |
| 6   | Guard Gallivant        | 246.9 us  | 54.072 ms  | [problem](https://adventofcode.com/2024/day/6) / [solution](2024/06/src/lib.rs)  |
| 7   | Bridge Repair          | 4.255 ms  | 136.255 ms | [problem](https://adventofcode.com/2024/day/7) / [solution](2024/07/src/lib.rs)  |
| 8   | Resonant Collinearity  | 22.9 us   | 37.3 us    | [problem](https://adventofcode.com/2024/day/8) / [solution](2024/08/src/lib.rs)  |
| 9   | Disk Fragmenter        | 869.7 us  | 39.733 ms  | [problem](https://adventofcode.com/2024/day/9) / [solution](2024/09/src/lib.rs)  |
| 10  | Hoof It                | 532.2 us  | 556.4 us   | [problem](https://adventofcode.com/2024/day/10) / [solution](2024/10/src/lib.rs) |
| 11  | Plutonian Pebbles      | 169.4 us  | 3.754 ms   | [problem](https://adventofcode.com/2024/day/11) / [solution](2024/11/src/lib.rs) |
| 12  | Garden Groups          | 1.838 ms  | 1.956 ms   | [problem](https://adventofcode.com/2024/day/12) / [solution](2024/12/src/lib.rs) |
| 13  | Claw Contraption       | 65.7 us   | 65.7 us    | [problem](https://adventofcode.com/2024/day/13) / [solution](2024/13/src/lib.rs) |
| 14  | Restroom Redoubt       | 57.0 us   | 18.658 ms  | [problem](https://adventofcode.com/2024/day/14) / [solution](2024/14/src/lib.rs) |
| 15  | Warehouse Woes         | 860.1 us  | 792.4 us   | [problem](https://adventofcode.com/2024/day/15) / [solution](2024/15/src/lib.rs) |
| 16  | Restroom Redoubt       | 57.0 us   | 18.658 ms  | [problem](https://adventofcode.com/2024/day/16) / [solution](2024/16/src/lib.rs) |
| 17  | Chronospatial Computer | 7.0 us    | 203.2 us   | [problem](https://adventofcode.com/2024/day/17) / [solution](2024/17/src/lib.rs) |
| 18  | RAM Run                | 266.2 us  | 319.655 ms | [problem](https://adventofcode.com/2024/day/18) / [solution](2024/18/src/lib.rs) |
| 19  | Linen Layout           | 25.916 ms | 25.919 ms  | [problem](https://adventofcode.com/2024/day/19) / [solution](2024/19/src/lib.rs) |
| 20  | Race Condition         | 1.035 ms  | 32.966 ms  | [problem](https://adventofcode.com/2024/day/20) / [solution](2024/20/src/lib.rs) |
| 21  | Keypad Conundrum       | 7.8 us    | 17.6 us    | [problem](https://adventofcode.com/2024/day/21) / [solution](2024/21/src/lib.rs) |

# Improvements

- Common data structures:
  - Map/Grid (2022/08, 2022/14)
  - Path (2022/09)
  - Coordinates (2022/09, 2022/14)
