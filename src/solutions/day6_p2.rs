use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let height = lines.len();
    let width = lines[0].len();

    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let move_checked = |x: (usize, usize), d: usize| {
        let d = DIRECTIONS[d];
        match (
            x.0.checked_add_signed(d.0 as isize),
            x.1.checked_add_signed(d.1 as isize),
        ) {
            (Some(i), Some(j)) => {
                if i >= height || j >= width {
                    None
                } else {
                    Some((i, j))
                }
            }
            _ => None,
        }
    };

    let mut start = (0, 0);
    let obstacles = lines
        .into_iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .fold(HashSet::new(), |mut acc, (j, ch)| {
                    if ch == '#' {
                        acc.insert((i, j));
                    } else if ch == '^' {
                        start = (i, j);
                    }

                    acc
                })
        })
        .collect::<HashSet<_>>();

    let mut cur = start;
    let mut cur_direction = 0;
    let mut visited = HashSet::new();
    let mut visited_with_direction = HashSet::new();

    let mut result = 0;

    'outer: loop {
        visited.insert(cur);

        let mut next_direction = cur_direction;

        let next = loop {
            let next = match move_checked(cur, next_direction) {
                Some(x) => x,
                None => break 'outer,
            };

            if !obstacles.contains(&next) {
                break next;
            }

            next_direction = (next_direction + 1) % 4;
        };

        if !visited.contains(&next) {
            let mut cur = cur;
            let mut cur_direction = next_direction;

            let mut obstacles = obstacles.clone();
            obstacles.insert(next);

            let mut visited_with_direction = visited_with_direction.clone();

            'inner: loop {
                if !visited_with_direction.insert((cur, cur_direction)) {
                    result += 1;
                    break 'inner;
                }

                cur = loop {
                    let next = match move_checked(cur, cur_direction) {
                        Some(x) => x,
                        None => break 'inner,
                    };

                    if !obstacles.contains(&next) {
                        break next;
                    }

                    cur_direction = (cur_direction + 1) % 4;
                };
            }
        }

        visited_with_direction.insert((cur, cur_direction));

        (cur, cur_direction) = (next, next_direction);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day6_p2() {
        let lines = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "6");
    }
}
