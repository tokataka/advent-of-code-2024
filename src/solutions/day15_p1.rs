use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Box,
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
                .map(|(j, ch)| match ch {
                    '#' => Wall,
                    '.' => Empty,
                    'O' => Box,
                    '@' => {
                        (robot_i, robot_j) = (i as i32, j as i32);
                        Robot
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let directions = HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);

    for attempt in attempts.chars() {
        let &(di, dj) = directions.get(&attempt).unwrap();

        let (mut ci, mut cj) = (robot_i + di, robot_j + dj);

        while map[ci as usize][cj as usize] == Box {
            ci += di;
            cj += dj;
        }

        if map[ci as usize][cj as usize] == Empty {
            map[ci as usize][cj as usize] = Box;
            map[robot_i as usize][robot_j as usize] = Empty;
            map[(robot_i + di) as usize][(robot_j + dj) as usize] = Robot;

            robot_i += di;
            robot_j += dj;
        }
    }

    map.into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.into_iter().enumerate().fold(0, |acc, (j, tile)| {
                acc + match tile {
                    Box => i * 100 + j,
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
    fn test_day15_p1() {
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

        assert_eq!(solution(lines), "10092");
    }
}
