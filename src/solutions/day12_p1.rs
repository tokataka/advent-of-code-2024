use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let height = lines.len();
    let width = lines[0].len();

    let map: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited = HashSet::new();

    let mut result = 0;

    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (i, line) in map.iter().enumerate() {
        for (j, &ch) in line.iter().enumerate() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let mut area = 0;
            let mut region = HashSet::new();
            let mut borders = HashSet::new();

            let mut st = vec![(i, j)];

            while let Some(cur) = st.pop() {
                if visited.contains(&cur) {
                    continue;
                }

                visited.insert(cur);
                region.insert(cur);

                area += 1;

                let next_candidates = DIRECTIONS.iter().enumerate().map(|(d, &(di, dj))| {
                    match (
                        cur.0.checked_add_signed(di as isize),
                        cur.1.checked_add_signed(dj as isize),
                    ) {
                        (Some(i), Some(j)) if i < height && j < width => (d, Some((i, j))),
                        _ => (d, None),
                    }
                });

                for (d, next) in next_candidates {
                    match next {
                        Some(next) if map[next.0][next.1] == ch => {
                            if !region.contains(&next) {
                                st.push(next);
                            }
                        }
                        _ => {
                            borders.insert((d, cur));
                        }
                    }
                }
            }

            result += area * borders.len() as i32;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day12_p1() {
        let lines = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "1930");
    }
}
