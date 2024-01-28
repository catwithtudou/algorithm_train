pub struct Solution;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        let mut vis = std::collections::HashSet::new();
        fn dfs(remain_x: i32, remain_y: i32, x: i32, y: i32, z: i32, vis: &mut std::collections::HashSet<(i32, i32)>) -> bool {
            let pair = (remain_x, remain_y);
            if vis.contains(&pair) {
                return false;
            }
            vis.insert(pair);
            if remain_x == z || remain_y == z || remain_x + remain_y == z {
                return true;
            }
            return dfs(remain_x, y, x, y, z, vis) ||
                dfs(x, remain_y, x, y, z, vis) ||
                dfs(0, remain_y, x, y, z, vis) ||
                dfs(remain_x, 0, x, y, z, vis) ||
                dfs(remain_x - remain_x.min(y - remain_y), remain_y + remain_x.min(y - remain_y), x, y, z, vis) ||
                dfs(remain_x + remain_y.min(x - remain_x), remain_y - remain_y.min(x - remain_x), x, y, z, vis);
        }
        dfs(0, 0, x, y, z, &mut vis)
    }
}