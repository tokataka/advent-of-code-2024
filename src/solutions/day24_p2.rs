use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut iter = lines.split(|x| x.is_empty());
    iter.next();

    let mut z_max = "";
    let mut gates = HashMap::new();

    iter.next().unwrap().iter().for_each(|x| {
        let mut iter = x.split(" -> ");

        let mut left = iter.next().unwrap().split(" ");

        let input1 = left.next().unwrap();
        let op = match left.next().unwrap() {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("Invalid op"),
        };
        let input2 = left.next().unwrap();

        let output = iter.next().unwrap();

        if let Some(z) = output.strip_prefix("z") {
            z_max = z_max.max(z);
        }

        let (input1, input2) = (input1.min(input2), input1.max(input2));

        gates.insert(output, (input1, input2, op));
    });

    let mut swaps: Vec<&str> = vec![];

    for (&output, &(input1, _, op)) in &gates {
        let output_ops = gates
            .iter()
            .filter(|(_, &(input1, input2, _))| [input1, input2].contains(&output))
            .map(|(_, &(_, _, op))| op)
            .collect::<HashSet<_>>();

        match (input1.strip_prefix("x"), output.strip_prefix("z")) {
            // (x00, y00) -> (z00):  first sum bit
            (Some("00"), Some("00")) => {
                if op == Op::Xor {
                    continue;
                }
            }
            // (x00, y00) -> (???): first carry bit skips full adder OR gate
            (Some("00"), None) => {
                if op == Op::And && output_ops == HashSet::from([Op::And, Op::Xor]) {
                    continue;
                }
            }
            // (x__, y__) -> (???): first half adder
            (Some(_), None) => {
                if op == Op::Xor && output_ops == HashSet::from([Op::And, Op::Xor]) {
                    continue;
                }

                if op == Op::And && output_ops == HashSet::from([Op::Or]) {
                    continue;
                }
            }
            // (???, ???) -> (z_max): last carry bit
            (None, Some(z)) if z == z_max => {
                if op == Op::Or {
                    continue;
                }
            }
            // (???, ???) -> (z__): second half adder sum bit
            (None, Some(_)) => {
                if op == Op::Xor {
                    continue;
                }
            }
            // (???, ???) -> (???): second half adder carry bit or full adder carry bit
            (None, None) => {
                if op == Op::And && output_ops == HashSet::from([Op::Or]) {
                    continue;
                }

                if op == Op::Or && output_ops == HashSet::from([Op::And, Op::Xor]) {
                    continue;
                }
            }
            // other cases are not allowed
            _ => {}
        };

        swaps.push(output);
    }

    swaps.sort();
    swaps.join(",").to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day24_p2() {
        let lines = "
x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "z00,z01,z02,z05");
    }
}
