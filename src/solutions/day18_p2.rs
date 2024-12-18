use std::collections::HashSet;

struct UnionFind {
    uf: Vec<Vec<(usize, usize)>>,
}

impl UnionFind {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            uf: (0..height)
                .map(|i| (0..width).map(|j| (i, j)).collect())
                .collect(),
        }
    }

    fn get(&self, (i, j): (usize, usize)) -> &(usize, usize) {
        &self.uf[i][j]
    }

    fn get_mut(&mut self, (i, j): (usize, usize)) -> &mut (usize, usize) {
        &mut self.uf[i][j]
    }

    pub fn find(&mut self, x: (usize, usize)) -> (usize, usize) {
        if *self.get(x) != x {
            *self.get_mut(x) = self.find(*self.get(x));
        }

        *self.get(x)
    }

    pub fn union(&mut self, a: (usize, usize), b: (usize, usize)) {
        let a = self.find(a);
        let b = self.find(b);

        let (a, b) = (a.min(b), a.max(b));

        *self.get_mut(b) = a;
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    #[cfg(test)]
    let width = 7;
    #[cfg(not(test))]
    let width = 71;
    let height = width;

    let fall_bytes = lines.iter().map(|line| {
        let (j, i) = line.split_once(',').unwrap();

        (i.parse::<usize>().unwrap(), j.parse::<usize>().unwrap())
    });

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut uf = UnionFind::new(width + 2, height + 2);
    let mut corrupted = HashSet::new();

    for i in 1..(height + 1) {
        uf.union((i, 0), (1, 0));
        uf.union((i, width + 1), (0, 1));

        corrupted.insert((i, 0));
        corrupted.insert((i, width + 1));
    }

    for j in 1..(width + 1) {
        uf.union((0, j), (0, 1));
        uf.union((height + 1, j), (1, 0));

        corrupted.insert((0, j));
        corrupted.insert((height + 1, j));
    }

    for (fall_i, fall_j) in fall_bytes {
        let (fall_i, fall_j) = (fall_i + 1, fall_j + 1);

        for (di, dj) in DIRECTIONS {
            let (i, j) = ((fall_i as i32 + di) as usize, (fall_j as i32 + dj) as usize);

            if corrupted.contains(&(i, j)) {
                uf.union((fall_i, fall_j), (i, j));
            }
        }

        if uf.find((0, 1)) == uf.find((1, 0)) {
            return format!("{},{}", fall_j - 1, fall_i - 1);
        }

        corrupted.insert((fall_i, fall_j));
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_day18_p2() {
        let lines = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
        "
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .collect();

        assert_eq!(solution(lines), "6,1");
    }
}
