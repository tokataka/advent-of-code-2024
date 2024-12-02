use std::collections::HashMap;

pub fn solution(lines: Vec<&str>) -> String {
    let (mut left, mut right) = (vec![], HashMap::new());

    for line in lines {
        let mut iter = line.split_whitespace();

        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        *right
            .entry(iter.next().unwrap().parse::<i32>().unwrap())
            .or_default() += 1;
    }

    let mut result = 0;

    for a in left {
        result += a * right.get(&a).unwrap_or(&0);
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day1_p2() {
        let lines = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "31");
    }
}
