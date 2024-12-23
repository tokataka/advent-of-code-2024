use std::collections::{HashMap, HashSet};

pub fn solution(lines: Vec<&str>) -> String {
    const MOD: u64 = 16777216;

    let mut cache: Vec<Option<u64>> = vec![None; 16777216];

    fn next_secret_number(secret_number: u64, cache: &mut [Option<u64>]) -> u64 {
        if let Some(x) = cache[secret_number as usize] {
            return x;
        }

        let mut result = secret_number;
        result = (result ^ (result * 64)) % MOD;
        result = (result ^ (result / 32)) % MOD;
        result = (result ^ (result * 2048)) % MOD;

        cache[secret_number as usize] = Some(result);
        result
    }

    let secret_numbers = lines
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut result: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();

    let prices_and_diffs = secret_numbers
        .into_iter()
        .map(|secret_number| {
            (0..2000)
                .fold(
                    (vec![], secret_number),
                    |(mut acc, last_secret_number), _| {
                        let next_secret_number = next_secret_number(last_secret_number, &mut cache);
                        let next_price = next_secret_number % 10;
                        let last_price = last_secret_number % 10;
                        let next_diff = next_price as i32 - last_price as i32;

                        acc.push((next_price, next_diff));
                        (acc, next_secret_number)
                    },
                )
                .0
        })
        .collect::<Vec<_>>();

    for price_and_diff in prices_and_diffs {
        let mut visited = HashSet::new();

        for x in price_and_diff.windows(4) {
            let diff_sequence = (x[0].1, x[1].1, x[2].1, x[3].1);
            let price = x[3].0;

            if visited.contains(&diff_sequence) {
                continue;
            }

            visited.insert(diff_sequence);

            result
                .entry(diff_sequence)
                .and_modify(|x| *x += price)
                .or_insert(price);
        }
    }

    result.values().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day22_p2() {
        let lines = "
1
2
3
2024
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "23");
    }
}
