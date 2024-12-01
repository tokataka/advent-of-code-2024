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
