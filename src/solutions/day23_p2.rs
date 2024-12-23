use std::collections::{HashMap, HashSet};

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    graph: &'a HashMap<&'a str, HashSet<&'a str>>,
) -> Vec<HashSet<&'a str>> {
    if p.is_empty() && x.is_empty() {
        return Vec::from([r]);
    }

    let mut all_cliques = Vec::new();

    for node in p.clone() {
        let node_neighbors = graph.get(node).unwrap();

        let mut new_r = r.clone();
        new_r.insert(node);

        let new_p = p.intersection(node_neighbors).cloned().collect();
        let new_x = x.intersection(node_neighbors).cloned().collect();

        all_cliques.extend(bron_kerbosch(new_r, new_p, new_x, graph).into_iter());

        p.remove(node);
        x.insert(node);
    }

    all_cliques
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut graph = HashMap::new();

    for line in lines {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let all_cliques = bron_kerbosch(
        HashSet::new(),
        HashSet::from_iter(graph.keys().cloned()),
        HashSet::new(),
        &graph,
    );

    let max_clique = all_cliques.iter().max_by_key(|x| x.len()).unwrap();

    let mut max_clique = max_clique.iter().cloned().collect::<Vec<_>>();
    max_clique.sort();

    max_clique.join(",")
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day23_p2() {
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

        assert_eq!(solution(lines), "co,de,ka,ta");
    }
}
