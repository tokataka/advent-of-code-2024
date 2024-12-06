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

## License

This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.
