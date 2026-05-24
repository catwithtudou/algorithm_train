pub struct Solution;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let d = d as usize;
        let mut memo = vec![0; n];

        fn dfs(i: usize, arr: &Vec<i32>, d: usize, memo: &mut Vec<i32>) -> i32 {
            if memo[i] > 0 {
                return memo[i];
            }

            let n = arr.len();
            let mut res = 0;

            let left = i.saturating_sub(d);
            let mut j = i;
            while j > left {
                j -= 1;
                if arr[j] >= arr[i] {
                    break;
                }
                res = res.max(dfs(j, arr, d, memo));
            }

            // 向右跳
            let right = (i + d).min(n - 1);
            for j in i + 1..=right {
                if arr[j] >= arr[i] {
                    break;
                }

                res = res.max(dfs(j, arr, d, memo));
            }

            memo[i] = res + 1;
            memo[i]
        }

        let mut ans = 0;

        for i in 0..n {
            ans = ans.max(dfs(i, &arr, d, &mut memo));
        }
        ans
    }
}
