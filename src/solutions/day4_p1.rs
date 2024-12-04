pub fn solution(lines: Vec<&str>) -> String {
    let lines = lines
        .into_iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];

    let height = lines.len();
    let width = lines[0].len();

    let mut result = 0;

    for i in 0..height {
        for j in 0..width {
            let i = i as i32;
            let j = j as i32;

            for &(di, dj) in &DIRECTIONS {
                if (0..4).all(|idx| {
                    if let Some(line) = lines.get((i + di * idx) as usize) {
                        if let Some(&cell) = line.get((j + dj * idx) as usize) {
                            return cell == TARGET[idx as usize];
                        }
                    }

                    false
                }) {
                    result += 1;
                }
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day4_p1() {
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

        assert_eq!(solution(lines), "18");
    }
}
