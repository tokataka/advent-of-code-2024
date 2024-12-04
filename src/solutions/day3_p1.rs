use regex::Regex;

pub fn solution(lines: Vec<&str>) -> String {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let line = lines.join(" ");

    mul_re
        .captures_iter(&line)
        .map(|c| {
            let (_, [a, b]) = c.extract();

            a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day3_p1() {
        let lines = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "161");
    }
}
