pub fn solution(lines: Vec<&str>) -> String {
    let [reg_lines, program_line] = lines.split(|x| x.is_empty()).collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let [mut reg_a, mut reg_b, mut reg_c] = reg_lines
        .iter()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<i32>>()[..]
    else {
        unreachable!()
    };

    let program = program_line[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut outputs: Vec<String> = vec![];

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
            3 => {
                if reg_a != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => reg_b ^= reg_c,
            5 => outputs.push((combo % 8).to_string()),
            6 => reg_b = reg_a / (1 << combo),
            7 => reg_c = reg_a / (1 << combo),
            _ => unreachable!(),
        };

        pointer += 2;
    }

    outputs.join(",").to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day17_p1() {
        let lines = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "4,6,3,5,6,3,5,2,1,0");
    }
}
