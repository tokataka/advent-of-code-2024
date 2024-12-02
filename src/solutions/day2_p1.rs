use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    lines
        .into_iter()
        .filter(|line| {
            let line_split = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let mut diffs = HashSet::new();

            for x in line_split.windows(2) {
                let diff = x[0] - x[1];

                if diff.abs() > 3 {
                    return false;
                }

                if diff == 0 {
                    return false;
                }

                diffs.insert(diff.signum());

                if diffs.len() > 1 {
                    return false;
                }
            }

            true
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day2_p1() {
        let lines = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "2");
    }
}
