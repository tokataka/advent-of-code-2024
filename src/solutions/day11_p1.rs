use std::collections::VecDeque;

pub fn solution(lines: Vec<&str>) -> String {
    let mut stones = lines[0]
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<VecDeque<_>>();

    for _ in 0..25 {
        let mut temp = VecDeque::new();

        while let Some(cur) = stones.pop_front() {
            if cur == 0 {
                temp.push_back(1);
                continue;
            }

            let mut digits = 0;

            while cur / 10u64.pow(digits) > 0 {
                digits += 1;
            }

            if digits % 2 == 1 {
                temp.push_back(cur * 2024);
            } else {
                temp.push_back(cur / 10u64.pow(digits / 2));
                temp.push_back(cur % 10u64.pow(digits / 2));
            }
        }

        stones = temp;
    }

    stones.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day11_p1() {
        let lines = "
125 17
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "55312");
    }
}
