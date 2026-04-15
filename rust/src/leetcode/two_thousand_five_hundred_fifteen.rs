pub struct Solution;

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;
        let mut ans = n;
        for i in 0..n {
            if words[i as usize] == target {
                let idx = (i - start_index).abs();
                ans = ans.min(idx.min(n - idx));
            }
        }
        if ans == n as i32 {
            -1
        } else {
            ans
        }
    }
}
