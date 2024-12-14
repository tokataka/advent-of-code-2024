use regex::Regex;

pub fn solution(lines: Vec<&str>) -> String {
    lines
        .split(|x| x.is_empty())
        .filter_map(|machine| {
            let machine = machine.join(" ");

            let re = Regex::new(r"\d+").unwrap();

            let values = re
                .find_iter(&machine)
                .map(|c| c.as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let [ax, ay, bx, by, px, py] = values[..] else {
                unreachable!()
            };

            fn divmod<T>(a: T, b: T) -> (T, T)
            where
                T: Copy + std::ops::Div<Output = T> + std::ops::Rem<Output = T>,
            {
                (a / b, a % b)
            }

            let a = match divmod(px * by - py * bx, ax * by - bx * ay) {
                (a, 0) if a >= 0 => a,
                _ => return None,
            };

            let b = match divmod(px - ax * a, bx) {
                (b, 0) if b >= 0 => b,
                _ => return None,
            };

            Some(3 * a + b)
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day13_p1() {
        let lines = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "480");
    }
}
