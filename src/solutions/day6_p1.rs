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

    'outer: loop {
        visited.insert(cur);

        cur = loop {
            let next = match move_checked(cur, cur_direction) {
                Some(x) => x,
                None => break 'outer,
            };

            if !obstacles.contains(&next) {
                break next;
            }

            cur_direction = (cur_direction + 1) % 4;
        };
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day6_p1() {
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

        assert_eq!(solution(lines), "41");
    }
}
