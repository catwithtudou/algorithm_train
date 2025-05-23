pub struct Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = nums.len();
        let mut g = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
        }

        fn dfs(x: usize, fa: i32, nums: &Vec<i32>, k: i32, g: &Vec<Vec<usize>>) -> (i64, i64) {
            let mut f0 = 0;
            let mut f1 = i64::MIN;
            for &y in &g[x] {
                if y as i32 != fa {
                    let (r0, r1) = dfs(y, x as i32, nums, k, g);
                    let temp_f0 = f0;
                    let temp_f1 = f1;
                    f0 = (temp_f0 + r0).max(temp_f1 + r1);
                    f1 = (temp_f1 + r0).max(temp_f0 + r1);
                }
            }

            (
                (f0 + nums[x] as i64).max(f1 + (nums[x] ^ k)as i64),
                (f1 + nums[x] as i64).max(f0 + (nums[x] ^ k)as i64 ),
            )
        }

        let (ans, _) = dfs(0, -1, &nums, k, &g);
        ans
    }
}
