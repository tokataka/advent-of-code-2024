pub fn solution(lines: Vec<&str>) -> String {
    lines
        .into_iter()
        .filter_map(|line| {
            let (result, nums) = line.split_once(':').unwrap();

            let result = result.parse::<i64>().unwrap();
            let nums = nums
                .trim()
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<_>>();

            let mut st = vec![(result, 0)];

            while let Some((remain, idx)) = st.pop() {
                if idx == nums.len() {
                    if remain == 0 {
                        return Some(result);
                    } else {
                        continue;
                    }
                }

                if remain <= 0 {
                    continue;
                }

                let num = nums[idx];

                if remain % num == 0 {
                    st.push((remain / num, idx + 1));
                }

                st.push((remain - num, idx + 1));

                let mut digits = 1;
                while num / digits != 0 {
                    digits *= 10;
                };

                if remain % digits == num {
                    st.push((remain / digits, idx + 1));
                }
            }

            None
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day7_p2() {
        let lines = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "11387");
    }
}
