pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        fn dfs(
            mut i: usize,
            left: i32,
            candidates: &Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            path: &mut Vec<i32>,
        ) {
            if left == 0 {
                ans.push(path.clone());
                return;
            }

            if i == candidates.len() {
                return;
            }

            let x = candidates[i];
            if x > left {
                return;
            }

            path.push(x);
            dfs(i + 1, left - x, candidates, ans, path);
            path.pop();

            i += 1;
            while i < candidates.len() && candidates[i] == x {
                i += 1;
            }
            dfs(i, left, candidates, ans, path);
        }
        let mut ans = vec![];
        let mut path = vec![];
        dfs(0, target, &candidates, &mut ans, &mut path);
        return ans;
    }
}
