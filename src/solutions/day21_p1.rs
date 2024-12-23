use std::collections::{HashMap, HashSet, VecDeque};

pub fn solution(lines: Vec<&str>) -> String {
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let numeric_pos_map: HashMap<char, (i32, i32)> = HashMap::from([
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
    ]);

    let pos_numeric_map: HashMap<(i32, i32), char> =
        numeric_pos_map.iter().map(|(k, v)| (*v, *k)).collect();

    let directional_pos_map: HashMap<char, (i32, i32)> = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    let pos_directional_map: HashMap<(i32, i32), char> =
        directional_pos_map.iter().map(|(k, v)| (*v, *k)).collect();

    let mut result = 0;

    for line in lines {
        let targets = line.chars().collect::<Vec<char>>();
        let mut queue = VecDeque::from([('A', 'A', 'A', 0, 0)]);
        let mut visited: HashSet<(char, char, char, usize)> = HashSet::from([('A', 'A', 'A', 0)]);

        while let Some((numeric, bot1, bot2, target_idx, move_count)) = queue.pop_front() {
            if target_idx == targets.len() {
                result += move_count * line[..3].trim_start_matches('0').parse::<i32>().unwrap();
                break;
            }

            // execute bot2
            if let Some((next_numeric, next_bot1, next_bot2, next_target_idx)) = match bot2 {
                'A' => match bot1 {
                    'A' => {
                        if numeric == targets[target_idx] {
                            queue.clear();
                            Some((numeric, bot1, bot2, target_idx + 1))
                        } else {
                            None
                        }
                    }
                    bot1 => {
                        let numeric_pos = numeric_pos_map.get(&numeric).unwrap();
                        let next_numeric_pos = match bot1 {
                            '^' => (numeric_pos.0 - 1, numeric_pos.1),
                            'v' => (numeric_pos.0 + 1, numeric_pos.1),
                            '<' => (numeric_pos.0, numeric_pos.1 - 1),
                            '>' => (numeric_pos.0, numeric_pos.1 + 1),
                            _ => unreachable!(),
                        };

                        if let Some(&next_numeric) = pos_numeric_map.get(&next_numeric_pos) {
                            Some((next_numeric, bot1, bot2, target_idx))
                        } else {
                            None
                        }
                    }
                },
                bot2 => {
                    let bot1_pos = directional_pos_map.get(&bot1).unwrap();
                    let next_bot1_pos = match bot2 {
                        '^' => (bot1_pos.0 - 1, bot1_pos.1),
                        'v' => (bot1_pos.0 + 1, bot1_pos.1),
                        '<' => (bot1_pos.0, bot1_pos.1 - 1),
                        '>' => (bot1_pos.0, bot1_pos.1 + 1),
                        _ => unreachable!(),
                    };

                    if let Some(&next_bot1) = pos_directional_map.get(&next_bot1_pos) {
                        Some((numeric, next_bot1, bot2, target_idx))
                    } else {
                        None
                    }
                }
            } {
                if !visited.contains(&(next_numeric, next_bot1, next_bot2, next_target_idx)) {
                    visited.insert((next_numeric, next_bot1, next_bot2, next_target_idx));
                    queue.push_back((
                        next_numeric,
                        next_bot1,
                        next_bot2,
                        next_target_idx,
                        move_count + 1,
                    ));
                }
            }

            // move bot2
            let bot2_pos = directional_pos_map.get(&bot2).unwrap();

            for (bot2_di, bot2_dj) in DIRECTIONS {
                let next_bot2_pos = (bot2_pos.0 + bot2_di, bot2_pos.1 + bot2_dj);
                let next_bot2 = match pos_directional_map.get(&next_bot2_pos) {
                    Some(x) => *x,
                    None => continue,
                };
                if !visited.contains(&(numeric, bot1, next_bot2, target_idx)) {
                    visited.insert((numeric, bot1, next_bot2, target_idx));
                    queue.push_back((numeric, bot1, next_bot2, target_idx, move_count + 1));
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day21_p1() {
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
