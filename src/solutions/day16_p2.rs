#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use Tile::*;

pub fn solution(lines: Vec<&str>) -> String {
    type Position = (i32, i32);

    let mut start = (0, 0);
    let mut end = (0, 0);

    const DIRECTIONS: [Position; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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

    let mut pq = BinaryHeap::from([(Reverse(0), start, 1)]);
    let mut score_map: HashMap<(Position, usize), i32> = HashMap::new();
    let mut visited_map: HashMap<(i32, Position, usize), HashSet<Position>> =
        HashMap::from([((0, start, 1), HashSet::from([start]))]);

    let mut best_score = i32::MAX;

    while let Some((Reverse(cur_score), cur, cur_direction)) = pq.pop() {
        if *score_map.entry((cur, cur_direction)).or_insert(cur_score) < cur_score {
            continue;
        }

        if cur == end {
            best_score = best_score.min(cur_score);
            continue;
        }

        let cur_visited = visited_map
            .get(&(cur_score, cur, cur_direction))
            .unwrap()
            .clone();

        [(0, 0), (1, 1000), (3, 1000)]
            .into_iter()
            .for_each(|(rotate, rotate_score)| {
                let next_direction = (cur_direction + rotate) % 4;
                let d = DIRECTIONS[next_direction];
                let next = (cur.0 + d.0, cur.1 + d.1);
                let next_score = cur_score + rotate_score + 1;

                if map[next.0 as usize][next.1 as usize] == Wall {
                    return;
                }

                if let Some(visited) = visited_map.get_mut(&(next_score, next, next_direction)) {
                    *visited = visited.union(&cur_visited).cloned().collect();
                    return;
                }

                let mut next_visited = cur_visited.clone();
                next_visited.insert(next);
                visited_map.insert((next_score, next, next_direction), next_visited);

                pq.push((Reverse(next_score), next, next_direction));
            });
    }

    (0..4)
        .filter_map(|d| visited_map.get(&(best_score, end, d)))
        .flat_map(|x| x.iter())
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day16_p2() {
        //         let lines = "
        // ###############
        // #.......#....E#
        // #.#.###.#.###.#
        // #.....#.#...#.#
        // #.###.#####.#.#
        // #.#.#.......#.#
        // #.#.#####.###.#
        // #...........#.#
        // ###.#.#####.#.#
        // #...#.....#.#.#
        // #.#.#.###.#.#.#
        // #.....#...#.#.#
        // #.###.#.#.#.#.#
        // #S..#.....#...#
        // ###############
        //         "
        //         .trim()
        //         .split('\n')
        //         .map(|x| x.trim())
        //         .collect();

        //         assert_eq!(solution(lines), "45");

        let lines = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "64");
    }
}
