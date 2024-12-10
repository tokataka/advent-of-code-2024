use std::collections::VecDeque;

pub fn solution(lines: Vec<&str>) -> String {
    let mut files = VecDeque::new();
    let mut empty_spaces = Vec::new();

    let mut idx: u64 = 0;
    let mut file_id: u64 = 0;
    let mut is_file_mode = true;

    for ch in lines[0].chars() {
        let count = ch as u64 - '0' as u64;

        if is_file_mode {
            files.push_back((file_id, idx, count));

            file_id += 1;
        } else {
            empty_spaces.push((idx, count));
        }

        idx += count;
        is_file_mode = !is_file_mode;
    }

    empty_spaces.sort();

    let mut result = 0;

    while let Some((file_id, idx, count)) = files.pop_back() {
        let start_idx = match empty_spaces
            .iter_mut()
            .filter(|x| x.0 < idx)
            .find(|x| x.1 >= count)
        {
            Some((move_idx, move_count)) => {
                let start_idx = *move_idx;

                *move_idx += count;
                *move_count -= count;

                start_idx
            }
            None => idx,
        };

        for i in start_idx..(start_idx + count) {
            result += file_id * i;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day9_p2() {
        let lines = "
2333133121414131402
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "2858");
    }
}
