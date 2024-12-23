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

    let mut secret_numbers = lines
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..2000 {
        secret_numbers
            .iter_mut()
            .for_each(|x| *x = next_secret_number(*x, &mut cache));
    }

    secret_numbers.into_iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day22_p1() {
        let lines = "
1
10
100
2024
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "37327623");
    }
}
