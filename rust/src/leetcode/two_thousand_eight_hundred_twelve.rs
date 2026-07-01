pub struct Solution;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let dirs: [(isize, isize); 4] = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ];

        let mut dis = vec![vec![-1; n]; n];
        let mut q: Vec<(usize, usize)> = Vec::new();

        // 多源 BFS：所有小偷位置作为起点，距离为 0
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] > 0 {
                    dis[i][j] = 0;
                    q.push((i, j));
                }
            }
        }

        let mut groups: Vec<Vec<(usize, usize)>> = vec![q.clone()];

        // 按照距离分层
        while !q.is_empty() {
            let tmp = std::mem::take(&mut q);

            for (i, j) in tmp {
                for &(dx, dy) in &dirs {
                    let x = i as isize + dx;
                    let y = j as isize + dy;

                    if x >= 0 && x < n as isize && y >= 0 && y < n as isize {
                        let x = x as usize;
                        let y = y as usize;

                        if dis[x][y] < 0 {
                            dis[x][y] = groups.len() as i32;
                            q.push((x, y));
                        }
                    }
                }
            }

            groups.push(q.clone());
        }

        let mut fa: Vec<usize> = (0..n * n).collect();

        fn find(fa: &mut Vec<usize>, x: usize) -> usize {
            if fa[x] != x {
                let root = find(fa, fa[x]);
                fa[x] = root;
            }
            fa[x]
        }

        // 从高安全系数往低安全系数枚举
        for ans in (1..=groups.len() - 2).rev() {
            for &(i, j) in &groups[ans] {
                for &(dx, dy) in &dirs {
                    let x = i as isize + dx;
                    let y = j as isize + dy;

                    if x >= 0 && x < n as isize && y >= 0 && y < n as isize {
                        let x = x as usize;
                        let y = y as usize;

                        if dis[x][y] >= ans as i32 {
                            let a = find(&mut fa, x * n + y);
                            let b = find(&mut fa, i * n + j);
                            fa[a] = b;
                        }
                    }
                }
            }

            let start = find(&mut fa, 0);
            let end = find(&mut fa, n * n - 1);

            if start == end {
                return ans as i32;
            }
        }

        0
    }
}