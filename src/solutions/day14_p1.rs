use std::collections::HashMap;

use regex::Regex;

pub fn solution(lines: Vec<&str>) -> String {
    // Note: Added width and height at the first line for convenience
    let (width, height) = lines[0]
        .split_once(' ')
        .map(|x| (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap()))
        .unwrap();

    lines
        .iter()
        .skip(1)
        .map(|line| {
            let re = Regex::new(r"-?\d+").unwrap();

            let values = re
                .find_iter(line)
                .map(|c| c.as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let [pj, pi, vj, vi] = values[..] else {
                unreachable!()
            };

            let result_i = (pi + vi * 100).rem_euclid(height);
            let result_j = (pj + vj * 100).rem_euclid(width);

            (result_i, result_j)
        })
        .fold(HashMap::<(i32, i32), i32>::new(), |mut acc, x| {
            *acc.entry(x).or_default() += 1;
            acc
        })
        .into_iter()
        .fold(HashMap::new(), |mut acc, x| {
            match x.0 {
                (i, j) if i < (height / 2) && j < (width / 2) => {
                    *acc.entry(0).or_default() += x.1;
                }

                (i, j) if i > (height / 2) && j < (width / 2) => {
                    *acc.entry(1).or_default() += x.1;
                }

                (i, j) if i < (height / 2) && j > (width / 2) => {
                    *acc.entry(2).or_default() += x.1;
                }

                (i, j) if i > (height / 2) && j > (width / 2) => {
                    *acc.entry(3).or_default() += x.1;
                }

                _ => {}
            }

            acc
        })
        .values()
        .product::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day14_p1() {
        let lines = "
11 7
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "12");
    }
}
