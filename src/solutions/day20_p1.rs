use std::collections::VecDeque;

pub fn solution(lines: Vec<&str>) -> String {
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let least_picoseconds = if cfg!(test) { 10 } else { 100 };

    let mut start = (0, 0);
    let mut end = (0, 0);

    let map = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| {
                    match ch {
                        'S' => start = (i, j),
                        'E' => end = (i, j),
                        _ => (),
                    }
                    ch
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut distances_from_end = vec![vec![-1; width as usize]; height as usize];
    distances_from_end[end.0][end.1] = 0;
    let mut distance_without_cheat = 0;

    let mut queue = VecDeque::from([end]);
    let mut visited = vec![vec![false; width as usize]; height as usize];

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        visited[cur_i][cur_j] = true;

        let cur_distance = distances_from_end[cur_i][cur_j];

        if cur_i == start.0 && cur_j == start.1 {
            distance_without_cheat = cur_distance;
            break;
        }

        for (di, dj) in DIRECTIONS {
            let new_i = cur_i as i32 + di;
            let new_j = cur_j as i32 + dj;

            if new_i < 0 || new_i >= height || new_j < 0 || new_j >= width {
                continue;
            }

            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if visited[new_i][new_j] {
                continue;
            }

            if map[new_i][new_j] == '#' {
                continue;
            }

            distances_from_end[new_i][new_j] = cur_distance + 1;
            queue.push_back((new_i, new_j));
        }
    }

    let mut distances_from_start = vec![vec![-1; width as usize]; height as usize];
    distances_from_start[start.0][start.1] = 0;

    let mut queue = VecDeque::from([start]);
    let mut visited = vec![vec![false; width as usize]; height as usize];

    let mut result = 0;

    while let Some((cur_i, cur_j)) = queue.pop_front() {
        visited[cur_i][cur_j] = true;

        let cur_distance = distances_from_start[cur_i][cur_j];

        for (cheat_di, cheat_dj) in DIRECTIONS.map(|(di, dj)| (di * 2, dj * 2)) {
            let cheat_i = cur_i as i32 + cheat_di;
            let cheat_j = cur_j as i32 + cheat_dj;

            if cheat_i < 0 || cheat_i >= height || cheat_j < 0 || cheat_j >= width {
                continue;
            }

            let cheat_i = cheat_i as usize;
            let cheat_j = cheat_j as usize;

            let cheat_distance = distances_from_end[cheat_i][cheat_j];

            if cheat_distance >= 0 {
                let time_save = distance_without_cheat
                    - (cheat_distance + cur_distance + cheat_di.abs() + cheat_dj.abs());

                if time_save >= least_picoseconds {
                    result += 1;
                }
            }
        }

        for (di, dj) in DIRECTIONS {
            let new_i = cur_i as i32 + di;
            let new_j = cur_j as i32 + dj;

            if new_i < 0 || new_i >= height || new_j < 0 || new_j >= width {
                continue;
            }

            let new_i = new_i as usize;
            let new_j = new_j as usize;

            if visited[new_i][new_j] {
                continue;
            }

            if map[new_i][new_j] == '#' {
                continue;
            }

            distances_from_start[new_i][new_j] = cur_distance + 1;

            queue.push_back((new_i, new_j));
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day20_p1() {
        let lines = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "10");
    }
}
