pub struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut ans = -1;
        let mut cur_time = 1;
        let mut vis_time = vec![0; n];
        for mut x in 0..n {
            let start_time = cur_time;
            while x < n && vis_time[x] == 0 {
                vis_time[x] = cur_time;
                cur_time += 1;
                x = edges[x] as usize;
            }
            if x < n && vis_time[x] >= start_time {
                ans=ans.max(cur_time-vis_time[x]);
            }
        }
        ans
    }
}
