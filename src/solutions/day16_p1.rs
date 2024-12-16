#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use Tile::*;

pub fn solution(lines: Vec<&str>) -> String {
    let mut start = (0, 0);
    let mut end = (0, 0);

    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let map = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| match ch {
                    '#' => Wall,
                    '.' => Empty,
                    'S' => {
                        start = (i as i32, j as i32);
                        Empty
                    }
                    'E' => {
                        end = (i as i32, j as i32);
                        Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let mut pq = BinaryHeap::from([Reverse((0, start, 1))]);
    let mut visited = HashSet::new();

    while let Some(Reverse((cur_score, cur, cur_direction))) = pq.pop() {
        if cur == end {
            return cur_score.to_string();
        }

        if visited.contains(&(cur, cur_direction)) {
            continue;
        }

        visited.insert((cur, cur_direction));

        [(0, 0), (1, 1000), (3, 1000)]
            .into_iter()
            .for_each(|(rotate, rotate_score)| {
                let next_direction = (cur_direction + rotate) % 4;
                let d = DIRECTIONS[next_direction as usize];
                let next = (cur.0 + d.0, cur.1 + d.1);

                if map[next.0 as usize][next.1 as usize] == Wall {
                    return;
                }

                pq.push(Reverse((
                    cur_score + rotate_score + 1,
                    next,
                    next_direction,
                )));
            });
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day16_p1() {
        let lines = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "7036");
    }
}
