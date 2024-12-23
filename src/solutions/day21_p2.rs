use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Maps {
    numeric_pos_map: HashMap<char, (i64, i64)>,
    directional_pos_map: HashMap<char, (i64, i64)>,
    pos_numeric_map: HashMap<(i64, i64), char>,
    pos_directional_map: HashMap<(i64, i64), char>,
}

impl Maps {
    pub fn new() -> Self {
        Self {
            numeric_pos_map: HashMap::from([
                ('7', (0, 0)),
                ('8', (0, 1)),
                ('9', (0, 2)),
                ('4', (1, 0)),
                ('5', (1, 1)),
                ('6', (1, 2)),
                ('1', (2, 0)),
                ('2', (2, 1)),
                ('3', (2, 2)),
                ('0', (3, 1)),
                ('A', (3, 2)),
            ]),
            pos_numeric_map: HashMap::from([
                ((0, 0), '7'),
                ((0, 1), '8'),
                ((0, 2), '9'),
                ((1, 0), '4'),
                ((1, 1), '5'),
                ((1, 2), '6'),
                ((2, 0), '1'),
                ((2, 1), '2'),
                ((2, 2), '3'),
                ((3, 1), '0'),
                ((3, 2), 'A'),
            ]),
            directional_pos_map: HashMap::from([
                ('^', (0, 1)),
                ('A', (0, 2)),
                ('<', (1, 0)),
                ('v', (1, 1)),
                ('>', (1, 2)),
            ]),
            pos_directional_map: HashMap::from([
                ((0, 1), '^'),
                ((0, 2), 'A'),
                ((1, 0), '<'),
                ((1, 1), 'v'),
                ((1, 2), '>'),
            ]),
        }
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    const DIRECTIONS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    const MAX_DEPTH: usize = 25;

    let mut cache: HashMap<(usize, char, char), usize> = HashMap::new();

    fn dp(
        depth: usize,
        from: char,
        to: char,
        cache: &mut HashMap<(usize, char, char), usize>,
        maps: &Maps,
    ) -> usize {
        if let Some(&result) = cache.get(&(depth, from, to)) {
            return result;
        }

        if from == to {
            return 0;
        }

        if depth > MAX_DEPTH {
            return 0;
        }

        let (char_pos_map, pos_char_map) = if depth == 0 {
            (&maps.numeric_pos_map, &maps.pos_numeric_map)
        } else {
            (&maps.directional_pos_map, &maps.pos_directional_map)
        };

        let mut pq = BinaryHeap::from([(Reverse(0), from, 'A')]);
        let mut costs: HashMap<(char, char), usize> = HashMap::new();

        let mut result = usize::MAX;

        while let Some((Reverse(cost), cur, last_bot)) = pq.pop() {
            if let Some(&last_cost) = costs.get(&(cur, last_bot)) {
                if last_cost <= cost {
                    continue;
                }
            }

            costs.insert((cur, last_bot), cost);

            if cur == to {
                let result_cost = cost + dp(depth + 1, last_bot, 'A', cache, maps);
                result = result.min(result_cost);
            }

            let &cur_pos = char_pos_map.get(&cur).unwrap();

            for direction in DIRECTIONS {
                let next_pos = (cur_pos.0 + direction.0, cur_pos.1 + direction.1);
                if let Some(&next) = pos_char_map.get(&next_pos) {
                    let direction_char = match &direction {
                        (0, 1) => '>',
                        (1, 0) => 'v',
                        (0, -1) => '<',
                        (-1, 0) => '^',
                        _ => unreachable!(),
                    };
                    let next_cost = cost + dp(depth + 1, last_bot, direction_char, cache, maps) + 1;
                    pq.push((Reverse(next_cost), next, direction_char));
                }
            }
        }

        cache.insert((depth, from, to), result);

        result
    }

    let maps = Maps::new();

    let mut result = 0;

    for line in lines {
        let mut line_result = 0;
        let numeric_part = line[..3].trim_start_matches('0').parse::<usize>().unwrap();

        let line = ['A'].into_iter().chain(line.chars()).collect::<Vec<_>>();

        for x in line.windows(2) {
            line_result += dp(0, x[0], x[1], &mut cache, &maps) + 1;
        }

        result += line_result * numeric_part;
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day21_p2_2() {
        let lines = "
029A
980A
179A
456A
379A
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "126384");
    }
}
