use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
    Robot,
}

use Tile::*;

pub fn solution(lines: Vec<&str>) -> String {
    let mut iter = lines.split(|x| x.is_empty());
    let map = iter.next().unwrap();
    let attempts = iter.next().unwrap().join("");

    let (mut robot_i, mut robot_j) = (0, 0);

    let mut map = map
        .iter()
        .enumerate()
        .map(|(i, &line)| {
            line.chars()
                .enumerate()
                .flat_map(|(j, ch)| match ch {
                    '#' => [Wall, Wall],
                    '.' => [Empty, Empty],
                    'O' => [BoxLeft, BoxRight],
                    '@' => {
                        (robot_i, robot_j) = (i as i32, 2 * j as i32);
                        [Robot, Empty]
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let directions = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);

    'outer: for attempt in attempts.chars() {
        let mut boxes = vec![];

        let &(di, dj) = directions.get(&attempt).unwrap();

        let mut q = VecDeque::from([(robot_i + di, robot_j + dj)]);
        let mut box_visited = HashSet::new();

        while let Some((ci, mut cj)) = q.pop_front() {
            match map[ci as usize][cj as usize] {
                BoxLeft => {}
                BoxRight => cj -= 1,
                Empty => continue,
                _ => continue 'outer,
            }

            if box_visited.contains(&(ci, cj)) {
                continue;
            }

            boxes.push((ci, cj));
            box_visited.insert((ci, cj));

            match attempt {
                '>' => q.push_back((ci, cj + 2)),
                '<' => q.push_back((ci, cj - 1)),
                '^' | 'v' => {
                    q.push_back((ci + di, cj));
                    q.push_back((ci + di, cj + 1));
                }
                _ => unreachable!(),
            }
        }

        while let Some((box_i, box_j)) = boxes.pop() {
            map[box_i as usize][box_j as usize] = Empty;
            map[box_i as usize][(box_j + 1) as usize] = Empty;
            map[(box_i + di) as usize][(box_j + dj) as usize] = BoxLeft;
            map[(box_i + di) as usize][(box_j + dj + 1) as usize] = BoxRight;
        }

        map[robot_i as usize][robot_j as usize] = Empty;
        map[(robot_i + di) as usize][(robot_j + dj) as usize] = Robot;

        robot_i += di;
        robot_j += dj;
    }

    map.into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.into_iter().enumerate().fold(0, |acc, (j, tile)| {
                acc + match tile {
                    BoxLeft => i * 100 + j,
                    _ => 0,
                }
            })
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day15_p2() {
        let lines = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "9021");
    }
}
