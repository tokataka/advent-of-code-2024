use std::collections::{HashMap, VecDeque};

pub fn solution(lines: Vec<&str>) -> String {
    #[cfg(test)]
    let width = 7;
    #[cfg(not(test))]
    let width = 71;
    let height = width;

    #[cfg(test)]
    let take_count = 12;
    #[cfg(not(test))]
    let take_count = 1024;

    let mut map = vec![vec![0; width]; height];

    lines.iter().take(take_count).for_each(|line| {
        let (j, i) = line.split_once(',').unwrap();

        let (i, j) = (i.parse::<usize>().unwrap(), j.parse::<usize>().unwrap());

        map[i][j] = 1;
    });

    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

    let mut distances = HashMap::from([((0, 0), 0)]);
    let mut q: VecDeque<(i32, i32)> = VecDeque::from([(0, 0)]);

    while let Some((ci, cj)) = q.pop_front() {
        let cur_distance = *distances.get(&(ci, cj)).unwrap();

        if ci == height as i32 - 1 && cj == width as i32 - 1 {
            return cur_distance.to_string();
        }

        for (di, dj) in DIRECTIONS {
            let (i, j) = (ci + di, cj + dj);

            if i < 0 || i >= height as i32 || j < 0 || j >= width as i32 {
                continue;
            }

            if map[i as usize][j as usize] == 1 {
                continue;
            }

            if distances.contains_key(&(i, j)) {
                continue;
            }

            distances.insert((i, j), cur_distance + 1);
            q.push_back((i, j));
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day18_p1() {
        let lines = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "22");
    }
}
