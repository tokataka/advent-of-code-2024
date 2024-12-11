use std::collections::{HashMap, HashSet};

pub fn solution(lines: Vec<&str>) -> String {
    let height = lines.len();
    let width = lines[0].len();

    let mut nodes = HashMap::new();

    lines.into_iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, ch)| {
            if ch == '.' {
                return;
            }

            nodes.entry(ch).or_insert(Vec::new()).push((i, j));
        });
    });

    let find_antinodes = |a: (usize, usize), b: (usize, usize)| {
        let (a_i, a_j) = (a.0 as i32, a.1 as i32);
        let (b_i, b_j) = (b.0 as i32, b.1 as i32);

        [
            (2 * a_i - b_i, 2 * a_j - b_j),
            (2 * b_i - a_i, 2 * b_j - a_j),
        ]
        .into_iter()
        .filter(|cur| cur.0 >= 0 && cur.0 < height as i32 && cur.1 >= 0 && cur.1 < width as i32)
        .collect::<HashSet<_>>()
    };

    nodes
        .values()
        .flat_map(|x| {
            let mut result = HashSet::new();

            for (i, &a) in x.iter().enumerate() {
                for &b in x.iter().skip(i + 1) {
                    result.extend(find_antinodes(a, b));
                }
            }

            result
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day8_p1() {
        let lines = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "14");
    }
}