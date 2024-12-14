use std::collections::HashSet;

use regex::Regex;

pub fn solution(lines: Vec<&str>) -> String {
    // Note: Added width and height at the first line for convenience
    let (width, height) = lines[0]
        .split_once(' ')
        .map(|x| (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap()))
        .unwrap();

    let mut robots = lines
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
            ((pi, pj), (vi, vj))
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    for count in 1.. {
        robots.iter_mut().for_each(|((pi, pj), (vi, vj))| {
            *pi = (*pi + *vi).rem_euclid(height);
            *pj = (*pj + *vj).rem_euclid(width);
        });

        let positions = robots.iter().map(|x| x.0).collect::<HashSet<_>>();

        if positions.len() == robots.len() {
            // let mut p = vec![vec![' '; width as usize]; height as usize];

            // positions.iter().for_each(|&(i, j)| {
            //     p[i as usize][j as usize] = '*';
            // });

            // for l in p {
            //     for c in l {
            //         print!("{c}");
            //     }
            //     println!();
            // }

            return count.to_string();
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day14_p2() {
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
