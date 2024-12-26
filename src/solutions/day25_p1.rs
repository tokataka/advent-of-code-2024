pub fn solution(lines: Vec<&str>) -> String {
    let (locks, keys) =
        lines
            .split(|x| x.is_empty())
            .fold((Vec::new(), Vec::new()), |(mut locks, mut keys), x| {
                if x[0] == "#####" {
                    locks.push(
                        (0..x[0].len())
                            .map(|col| {
                                let mut height = 1;
                                while x[height].chars().nth(col).unwrap() == '#' {
                                    height += 1;
                                }

                                height
                            })
                            .collect::<Vec<_>>(),
                    );
                } else {
                    keys.push(
                        (0..x[0].len())
                            .map(|col| {
                                let mut height = 1;
                                while x[x.len() - 1 - height].chars().nth(col).unwrap() == '#' {
                                    height += 1;
                                }

                                height
                            })
                            .collect::<Vec<_>>(),
                    );
                }
                (locks, keys)
            });
    let mut result = 0;

    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(l, k)| {
                l + k <= 7
            }) {
                result += 1;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day25_p1() {
        let lines = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "3");
    }
}
