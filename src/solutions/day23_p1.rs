use std::collections::{HashMap, HashSet};

pub fn solution(lines: Vec<&str>) -> String {
    let mut nodes_starts_t = HashSet::new();

    let nodes = lines
        .into_iter()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();

            if a.starts_with('t') {
                nodes_starts_t.insert(a);
            }

            if b.starts_with('t') {
                nodes_starts_t.insert(b);
            }

            (a, b)
        })
        .collect::<Vec<_>>();

    let mut graph = HashMap::new();

    for &(a, b) in nodes.iter() {
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let mut result = 0;

    for (&node, vertices) in graph.iter() {
        if vertices.len() < 2 {
            continue;
        }

        for (i, vertex_a) in vertices.iter().enumerate() {
            for vertex_b in vertices.iter().skip(i + 1) {
                if (nodes_starts_t.contains(node)
                    || nodes_starts_t.contains(vertex_a)
                    || nodes_starts_t.contains(vertex_b))
                    && graph.get(vertex_a).unwrap().contains(vertex_b)
                {
                    result += 1;
                }
            }
        }
    }

    (result / 3).to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day23_p1() {
        let lines = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "7");
    }
}
