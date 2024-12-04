pub fn solution(lines: Vec<&str>) -> String {
    let lines = lines
        .into_iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    const TARGET_IDX: [(usize, usize); 5] = [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)];
    const TARGET_CHAR: [[char; 5]; 4] = [
        ['M', 'M', 'A', 'S', 'S'],
        ['S', 'S', 'A', 'M', 'M'],
        ['M', 'S', 'A', 'M', 'S'],
        ['S', 'M', 'A', 'S', 'M'],
    ];

    let height = lines.len();
    let width = lines[0].len();

    let mut result = 0;

    for i in 0..(height - 2) {
        for j in 0..(width - 2) {
            let chars = (0..5)
                .map(|idx| {
                    let (di, dj) = TARGET_IDX[idx];
                    lines[i + di][j + dj]
                })
                .collect::<Vec<_>>();

            if TARGET_CHAR.iter().any(|target| target == &chars[..]) {
                result += 1;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day4_p2() {
        let lines = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "9");
    }
}
