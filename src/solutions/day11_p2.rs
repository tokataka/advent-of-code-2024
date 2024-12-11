use std::collections::HashMap;

pub fn solution(lines: Vec<&str>) -> String {
    let mut stones = lines[0].split(' ').map(|x| x.parse::<u64>().unwrap()).fold(
        HashMap::<u64, u64>::new(),
        |mut acc, x| {
            *acc.entry(x).or_default() += 1;

            acc
        },
    );

    for _ in 0..75 {
        let mut temp = HashMap::new();

        for (stone, count) in stones.into_iter() {
            if stone == 0 {
                *temp.entry(1).or_default() += count;
                continue;
            }

            let mut digits = 0;

            while stone / 10u64.pow(digits) > 0 {
                digits += 1;
            }

            if digits % 2 == 1 {
                *temp.entry(stone * 2024).or_default() += count;
            } else {
                *temp.entry(stone / 10u64.pow(digits / 2)).or_default() += count;
                *temp.entry(stone % 10u64.pow(digits / 2)).or_default() += count;
            }
        }

        stones = temp;
    }

    stones.values().sum::<u64>().to_string()
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

        assert_eq!(solution(lines), "65601038650482");
    }
}
