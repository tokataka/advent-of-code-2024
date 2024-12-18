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
| day1_p1       |     0.071 ms |     0.066 ms |     0.132 ms |
| day1_p2       |     0.093 ms |     0.089 ms |     0.148 ms |
| day2_p1       |     0.205 ms |     0.195 ms |     0.309 ms |
| day2_p2       |     0.449 ms |     0.437 ms |     0.580 ms |
| day3_p1       |     0.437 ms |     0.417 ms |     0.864 ms |
| day3_p2       |     0.557 ms |     0.538 ms |     0.717 ms |
| day4_p1       |     0.475 ms |     0.460 ms |     0.594 ms |
| day4_p2       |     0.420 ms |     0.414 ms |     0.531 ms |
| day5_p1       |     0.143 ms |     0.136 ms |     0.247 ms |
| day5_p2       |     0.260 ms |     0.252 ms |     0.357 ms |
| day6_p1       |     0.515 ms |     0.494 ms |     0.608 ms |
| day6_p2       |   298.857 ms |   292.509 ms |   308.852 ms |
| day7_p1       |     0.583 ms |     0.572 ms |     0.753 ms |
| day7_p2       |     1.103 ms |     1.083 ms |     1.337 ms |
| day8_p1       |     0.060 ms |     0.057 ms |     0.099 ms |
| day8_p2       |     0.269 ms |     0.255 ms |     0.440 ms |
| day9_p1       |     0.535 ms |     0.510 ms |     1.361 ms |
| day9_p2       |    33.306 ms |    32.958 ms |    36.338 ms |
| day10_p1      |     0.789 ms |     0.775 ms |     0.936 ms |
| day10_p2      |     0.697 ms |     0.682 ms |     0.826 ms |
| day11_p1      |     4.754 ms |     4.675 ms |     5.313 ms |
| day11_p2      |    10.449 ms |    10.327 ms |    10.941 ms |
| day12_p1      |     6.258 ms |     6.182 ms |     6.797 ms |
| day12_p2      |     8.502 ms |     8.406 ms |     9.272 ms |
| day13_p1      |    21.320 ms |    21.076 ms |    22.936 ms |
| day13_p2      |    21.296 ms |    21.102 ms |    22.040 ms |
| day14_p1      |    36.899 ms |    36.535 ms |    39.173 ms |
| day14_p2      |   123.583 ms |   122.650 ms |   129.318 ms |
| day15_p1      |     0.445 ms |     0.436 ms |     0.563 ms |
| day15_p2      |     1.219 ms |     1.191 ms |     1.364 ms |
| day16_p1      |     2.622 ms |     2.594 ms |     2.829 ms |
| day16_p2      |    66.806 ms |    65.877 ms |    71.600 ms |
| day17_p1      |     0.001 ms |     0.001 ms |     0.019 ms |
| day17_p2      |     0.009 ms |     0.008 ms |     0.023 ms |
| day18_p1      |     0.655 ms |     0.636 ms |     0.796 ms |
| day18_p2      |     0.781 ms |     0.755 ms |     0.956 ms |

## License

This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
