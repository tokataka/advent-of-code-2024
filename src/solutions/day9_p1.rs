use std::collections::VecDeque;

pub fn solution(lines: Vec<&str>) -> String {
    let mut files = VecDeque::new();

    let mut file_id: usize = 0;
    let mut is_file_mode = true;

    for ch in lines[0].chars() {
        let count = ch as usize - '0' as usize;

        if is_file_mode {
            for _ in 0..count {
                files.push_back(Some(file_id));
            }

            file_id += 1;
        } else {
            for _ in 0..count {
                files.push_back(None);
            }
        }

        is_file_mode = !is_file_mode;
    }

    let mut result = 0;

    for idx in 0.. {
        if files.is_empty() {
            break;
        }

        let cur = files.pop_front().unwrap();

        if let Some(cur) = cur {
            result += cur * idx;
        } else {
            let cur = loop {
                if let Some(Some(x)) = files.pop_back() {
                    break x;
                }
            };

            result += cur * idx;
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day9_p1() {
        let lines = "
2333133121414131402
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "1928");
    }
}
