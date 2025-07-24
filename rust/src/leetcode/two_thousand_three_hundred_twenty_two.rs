pub struct Solution;

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut g = vec![Vec::new(); n];

        // 构建邻接表
        for edge in &edges {
            let x = edge[0] as usize;
            let y = edge[1] as usize;
            g[x].push(y);
            g[y].push(x);
        }

        let mut xor = vec![0; n];
        let mut in_time = vec![0; n];
        let mut out_time = vec![0; n];
        let mut clock = 0;

        // DFS函数
        fn dfs(
            x: usize,
            fa: Option<usize>,
            g: &Vec<Vec<usize>>,
            nums: &Vec<i32>,
            xor: &mut Vec<i32>,
            in_time: &mut Vec<i32>,
            out_time: &mut Vec<i32>,
            clock: &mut i32,
        ) {
            *clock += 1;
            in_time[x] = *clock;
            xor[x] = nums[x];

            for &y in &g[x] {
                if Some(y) != fa {
                    dfs(y, Some(x), g, nums, xor, in_time, out_time, clock);
                    xor[x] ^= xor[y];
                }
            }
            out_time[x] = *clock;
        }

        dfs(0, None, &g, &nums, &mut xor, &mut in_time, &mut out_time, &mut clock);

        // 判断是否为祖先关系
        let is_ancestor = |x: usize, y: usize| -> bool {
            in_time[x] < in_time[y] && in_time[y] <= out_time[x]
        };

        let mut ans = i32::MAX;

        for x in 2..n {
            for y in 1..x {
                let (a, b, c) = if is_ancestor(x, y) {
                    (xor[y], xor[x] ^ xor[y], xor[0] ^ xor[x])
                } else if is_ancestor(y, x) {
                    (xor[x], xor[y] ^ xor[x], xor[0] ^ xor[y])
                } else {
                    (xor[x], xor[y], xor[0] ^ xor[x] ^ xor[y])
                };

                let max_val = a.max(b.max(c));
                let min_val = a.min(b.min(c));
                ans = ans.min(max_val - min_val);

                if ans == 0 {
                    return 0;
                }
            }
        }

        ans
    }
}