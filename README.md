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

| Solution Name |        Average |            Min |            Max |
| ------------- | -------------: | -------------: | -------------: |
| day1_p1       |       71.027µs |       66.629µs |      125.422µs |
| day1_p2       |       83.212µs |       77.049µs |      148.063µs |
| day2_p1       |      199.137µs |      186.824µs |      292.658µs |
| day2_p2       |      417.087µs |      409.096µs |      594.577µs |
| day3_p1       |      429.633µs |      410.018µs |      836.522µs |
| day3_p2       |      548.941µs |      534.497µs |      670.922µs |
| day4_p1       |      443.512µs |      428.098µs |      537.830µs |
| day4_p2       |      424.272µs |      413.769µs |      566.566µs |
| day5_p1       |      208.836µs |      138.924µs |      346.127µs |
| day5_p2       |      243.971µs |      236.674µs |      260.332µs |

## License

This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
