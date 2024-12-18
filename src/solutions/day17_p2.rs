use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pub reg_a: u64,
    pub loop_count: usize,
}

impl State {
    pub fn new(reg_a: u64, loop_count: usize) -> Self {
        Self { reg_a, loop_count }
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    let [_, program_line] = lines.split(|x| x.is_empty()).collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let program = program_line[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut output_reversed = program.clone();
    output_reversed.reverse();

    let mut q = VecDeque::from([State::new(0, 0)]);

    while let Some(state) = q.pop_front() {
        if state.loop_count >= output_reversed.len() {
            return state.reg_a.to_string();
        }

        // always 'reg_a <- reg_a / 8' mutates reg_a.
        'outer: for prev_reg_a in (state.reg_a * 8)..((state.reg_a + 1) * 8) {
            if prev_reg_a == 0 {
                continue;
            }

            // reg_b, reg_c never affected by previous reg_b, reg_c
            let mut reg_a = prev_reg_a;
            let mut reg_b = 0;
            let mut reg_c = 0;

            let mut pointer = 0;

            while pointer < program.len() {
                let (opcode, operand) = (program[pointer], program[pointer + 1]);

                let combo = match operand {
                    x @ 0..=3 => x,
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => unreachable!(),
                };

                match opcode {
                    0 => reg_a /= 1 << combo,
                    1 => reg_b ^= operand,
                    2 => reg_b = combo % 8,
                    // loop always occurs
                    3 => break,
                    4 => reg_b ^= reg_c,
                    5 => {
                        if combo % 8 != output_reversed[state.loop_count] {
                            continue 'outer;
                        }
                    }
                    6 => reg_b = reg_a / (1 << combo),
                    7 => reg_c = reg_a / (1 << combo),
                    _ => unreachable!(),
                };

                pointer += 2;
            }

            q.push_back(State::new(prev_reg_a, state.loop_count + 1));
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day17_p1() {
        let lines = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "117440");
    }
}
