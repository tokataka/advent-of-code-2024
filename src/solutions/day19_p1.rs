use std::collections::HashSet;

pub fn solution(lines: Vec<&str>) -> String {
    let [towels, onsens] = lines.split(|&x| x.is_empty()).collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let towels = towels[0].split(", ").collect::<HashSet<_>>();
    let max_towel_len = towels.iter().map(|x| x.len()).max().unwrap();

    let mut result = 0;

    'outer: for &onsen in onsens {
        let mut st = vec![0];
        let mut visited = HashSet::new();

        while let Some(cur) = st.pop() {
            if visited.contains(&cur) {
                continue;
            }

            visited.insert(cur);

            let remain = &onsen[cur..];

            for towel_len in 1..=(max_towel_len.min(remain.len())) {
                if towels.contains(&remain[..towel_len]) {
                    let next = cur + towel_len;

                    if next == onsen.len() {
                        result += 1;
                        continue 'outer;
                    }

                    st.push(next);
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
    fn test_day19_p1() {
        let lines = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "6");
    }
}
