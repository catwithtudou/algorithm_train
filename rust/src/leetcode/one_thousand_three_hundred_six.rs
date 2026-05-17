pub struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        fn dfs(i: usize, arr: &[i32], vis: &mut [bool]) -> bool {
            if i >= arr.len() || vis[i] {
                return false;
            }

            vis[i] = true;

            if arr[i] == 0 {
                return true;
            }

            dfs(i + arr[i] as usize, arr, vis) || dfs(i - arr[i] as usize, arr, vis)
        }

        dfs(start as usize, &arr, &mut vec![false; arr.len()])
    }
}
