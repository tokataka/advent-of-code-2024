use std::{cmp::Ordering, collections::HashSet};

pub fn solution(lines: Vec<&str>) -> String {
    let split_idx = lines.iter().position(|x| x.is_empty()).unwrap();
    let (rules, updates) = lines.split_at(split_idx);
    let updates = &updates[1..];

    let rules = rules
        .iter()
        .map(|rule| {
            (
                rule[0..2].parse::<i32>().unwrap(),
                rule[3..5].parse::<i32>().unwrap(),
            )
        })
        .collect::<HashSet<_>>();

    updates
        .iter()
        .map(|update| {
            update
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|numbers| {
            !numbers.is_sorted_by(|&a, &b| !rules.contains(&(b, a)))
        })
        .map(|mut numbers| {
            numbers.sort_by(|&a, &b| {
                if rules.contains(&(a, b)) {
                    Ordering::Less
                } else if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            numbers[numbers.len() / 2]
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day5_p2() {
        let lines = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "123");
    }
}
