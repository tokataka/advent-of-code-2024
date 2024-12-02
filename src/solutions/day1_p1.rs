pub fn solution(lines: Vec<&str>) -> String {
    let (mut left, mut right) = (vec![], vec![]);

    for line in lines {
        let mut iter = line.split_whitespace();

        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        right.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut result = 0;

    for (a, b) in left.into_iter().zip(right) {
        result += (a - b).abs();
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day1_p1() {
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

        assert_eq!(solution(lines), "11");
    }
}
