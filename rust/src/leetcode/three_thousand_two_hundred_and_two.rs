pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let k = k as usize;
        let mut f = vec![vec![0; k]; k];

        for x in nums {
            let x = (x % k as i32) as usize;
            for y in 0..k {
                let fxy = f[x][y];
                f[y][x] = fxy + 1;
                ans = ans.max(f[y][x]);
            }
        }
        ans
    }
}
