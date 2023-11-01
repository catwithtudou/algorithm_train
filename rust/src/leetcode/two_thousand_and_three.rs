use std::collections::HashSet;

pub struct Solution;


impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len();
        let mut ans = vec![1; n];
        let mut node = match nums.iter().position(|&x| x == 1) {
            Some(i) => i as i32,
            None => return ans,
        };

        // 建树
        let mut g = vec![vec![]; n];
        for i in 1..n {
            g[parents[i] as usize].push(i);
        }

        // 遍历 x 子树
        fn dfs(x: usize, g: &Vec<Vec<usize>>, vis: &mut HashSet<i32>, nums: &Vec<i32>) {
            vis.insert(nums[x]);
            for &son in &g[x] {
                if !vis.contains(&nums[son]) {
                    dfs(son, g, vis, nums);
                }
            }
        }

        let mut visited = HashSet::new();
        let mut mex = 2;
        while node != -1 {
            dfs(node as usize, &g, &mut visited, &nums);
            while visited.contains(&mex) {
                mex += 1;
            }
            ans[node as usize] = mex;
            node = parents[node as usize];
        }
        ans
    }
}

#[cfg(test)]
mod two_thousand_and_three_test {
    use super::*;

    #[test]
    fn two_thousand_and_three() {
        assert_eq!(Solution::smallest_missing_value_subtree(vec![-1, 2, 3, 0, 2, 4, 1], vec![2, 3, 4, 5, 6, 7, 8]), vec![1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(Solution::smallest_missing_value_subtree(vec![-1, 0, 1, 0, 3, 3], vec![5, 4, 6, 2, 1, 3]), vec![7, 1, 1, 4, 2, 1]);
    }
}