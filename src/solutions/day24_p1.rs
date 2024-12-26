use std::collections::HashMap;

pub fn solution(lines: Vec<&str>) -> String {
    let mut iter = lines.split(|x| x.is_empty());

    let mut cache = HashMap::new();

    iter.next().unwrap().iter().for_each(|x| {
        let mut iter = x.split(": ");
        let wire = iter.next().unwrap();
        let value = iter.next().unwrap().parse::<u64>().unwrap();

        cache.insert(wire, value);
    });

    let mut z_wires = vec![];

    let gates = iter
        .next()
        .unwrap()
        .iter()
        .map(|x| {
            let mut iter = x.split(" -> ");

            let mut left = iter.next().unwrap().split(" ");

            let input1 = left.next().unwrap();
            let gate = left.next().unwrap();
            let input2 = left.next().unwrap();

            let output = iter.next().unwrap();

            if output.starts_with("z") {
                z_wires.push(output);
            }

            (output, (input1, input2, gate))
        })
        .collect::<HashMap<_, _>>();

    fn dp<'a>(
        wire: &'a str,
        cache: &mut HashMap<&'a str, u64>,
        gates: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    ) -> u64 {
        if let Some(&value) = cache.get(wire) {
            return value;
        }

        let (input1, input2, gate) = *gates.get(wire).unwrap();

        let input1 = dp(input1, cache, gates);
        let input2 = dp(input2, cache, gates);

        let value = match gate {
            "AND" => input1 & input2,
            "OR" => input1 | input2,
            "XOR" => input1 ^ input2,
            _ => unreachable!(),
        };

        cache.insert(wire, value);

        value
    }

    let mut result = 0;

    z_wires.sort_unstable_by(|a, b| b.cmp(a));

    for wire in z_wires {
        result = (result << 1) | dp(wire, &mut cache, &gates);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day24_p1() {
        let lines = "
x00: 1
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "2024");
    }
}
