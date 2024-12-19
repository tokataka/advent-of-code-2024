use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn solution(lines: Vec<&str>) -> String {
    let [towels, onsens] = lines.split(|&x| x.is_empty()).collect::<Vec<_>>()[..] else {
        unreachable!()
    };

    let towels = towels[0].split(", ").collect::<HashSet<_>>();
    let max_towel_len = towels.iter().map(|x| x.len()).max().unwrap();

    let mut result = 0;

    for &onsen in onsens {
        let mut heap = BinaryHeap::from([Reverse(0)]);
        let mut counts: HashMap<usize, u64> = HashMap::from([(0, 1)]);

        while let Some(Reverse(cur)) = heap.pop() {
            if cur == onsen.len() {
                result += counts.get(&cur).unwrap();
                break;
            }

            let remain = &onsen[cur..];

            for towel_len in 1..=(max_towel_len.min(remain.len())) {
                if towels.contains(&remain[..towel_len]) {
                    let next = cur + towel_len;
                    let cur_count = *counts.get(&cur).unwrap();

                    if let Some(next_count) = counts.get_mut(&next) {
                        *next_count += cur_count;
                    } else {
                        counts.insert(next, cur_count);
                        heap.push(Reverse(next));
                    }
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
    fn test_day19_p2() {
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

        assert_eq!(solution(lines), "16");
    }
}
