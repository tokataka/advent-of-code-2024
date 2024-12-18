# Advent of Code 2024 Solutions in Rust

My solutions for [AoC 2024](https://adventofcode.com/2024) in Rust.

I created this project so that I could focus on solving the problem with less effort to other than problem solving.

## usage

```sh
# run
cargo run [--release] -- day1_p1

# benchmark
cargo run [--release] --bin bench [-- day1_p1 [day1_p2 ...]]
```

## How to add solutions

All solutions are named in {name}\_{part} format.

Solutions with same name share one input file.

You might want to add

- `src/solutions/{name}_{part}.rs` which includes following function
  ```rust
  pub fn solution(lines: Vec<&str>) -> String {
      ...
  }
  ```
- `resource/input/{name}.txt`
- solution name in `src/solutions/mod.rs` with comma seperated
  ```rust
  export_solutions!(
      day1_p1, day1_p2, ...
  );
  ```

## Benchmark Results (Ryzen 5 3600)

Result of 100 iterations

| Solution Name |      Average |          Min |          Max |
| ------------- | -----------: | -----------: | -----------: |
| day1_p1       |    67.544 µs |    63.714 µs |   121.268 µs |
| day1_p2       |    93.193 µs |    87.721 µs |   145.936 µs |
| day2_p1       |   201.832 µs |   195.262 µs |   310.176 µs |
| day2_p2       |   446.662 µs |   431.746 µs |   561.924 µs |
| day3_p1       |   429.380 µs |   406.597 µs |   846.309 µs |
| day3_p2       |   553.053 µs |   533.833 µs |   658.066 µs |
| day4_p1       |   479.770 µs |   458.598 µs |   628.619 µs |
| day4_p2       |   417.769 µs |   408.203 µs |   534.185 µs |
| day5_p1       |   144.611 µs |   139.429 µs |   201.659 µs |
| day5_p2       |   286.285 µs |   277.225 µs |   396.801 µs |
| day6_p1       |   511.597 µs |   491.046 µs |   632.795 µs |
| day6_p2       |   297.131 ms |   289.592 ms |   306.489 ms |
| day7_p1       |   575.713 µs |   565.632 µs |   707.213 µs |
| day7_p2       |     1.105 ms |     1.089 ms |     1.314 ms |
| day8_p1       |    61.161 µs |    57.063 µs |   119.111 µs |
| day8_p2       |   262.474 µs |   254.062 µs |   371.139 µs |
| day9_p1       |   548.437 µs |   506.757 µs |     1.386 ms |
| day9_p2       |    33.638 ms |    32.957 ms |    39.391 ms |
| day10_p1      |   756.239 µs |   734.854 µs |   948.868 µs |
| day10_p2      |   680.947 µs |   666.514 µs |   790.544 µs |
| day11_p1      |     5.920 ms |     5.724 ms |     7.831 ms |
| day11_p2      |    10.613 ms |    10.316 ms |    11.999 ms |
| day12_p1      |     6.737 ms |     6.588 ms |     7.175 ms |
| day12_p2      |     8.962 ms |     8.818 ms |     9.862 ms |
| day13_p1      |   106.806 ms |   105.443 ms |   110.293 ms |
| day13_p2      |   106.666 ms |   105.480 ms |   111.455 ms |
| day14_p1      |    37.140 ms |    36.601 ms |    39.837 ms |
| day14_p2      |   123.209 ms |   122.309 ms |   128.192 ms |
| day15_p1      |   449.884 µs |   434.597 µs |   638.870 µs |
| day15_p2      |     1.478 ms |     1.443 ms |     1.716 ms |
| day16_p1      |     2.950 ms |     2.875 ms |     3.178 ms |
| day16_p2      |    66.359 ms |    65.207 ms |    74.299 ms |
| day17_p1      |     1.709 µs |     1.150 µs |    33.354 µs |
| day17_p2      |     8.850 µs |     8.258 µs |    22.927 µs |

## License

This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
