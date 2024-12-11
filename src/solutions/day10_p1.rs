use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let start_list = lines
        .iter()
        .enumerate()
        .flat_map(|(i, &line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '0')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect::<Vec<_>>();

    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

    let mut result = 0;

    for start in start_list {
        let mut peaks = HashSet::new();

        let mut st = vec![(start, 0)];

        while let Some(((ci, cj), cur_h)) = st.pop() {
            if cur_h == 9 {
                peaks.insert((ci, cj));
                continue;
            }

            for (di, dj) in DIRECTIONS {
                let i = ci + di;
                let j = cj + dj;

                if i < 0 || i >= height || j < 0 || j >= width {
                    continue;
                }

                let next_h = (lines[i as usize].chars().nth(j as usize).unwrap() as i32) - ('0' as i32);

                if next_h == cur_h + 1 {
                    st.push(((i, j), next_h));
                }
            }
        }

        result += peaks.len()
    }


    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day10_p1() {
        let lines = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "36");
    }
}
