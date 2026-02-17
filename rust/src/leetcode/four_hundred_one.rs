pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut ans = vec![];
        for i in 0..12 {
            for j in 0..60 {
                if (i as u32).count_ones() + (j as u32).count_ones() == turned_on as u32  {
                    ans.push(format!("{:01}:{:02}", i, j));
                }
            }
        }
        ans
    }
}