pub struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut left = 0;
        let mut ans = 0;

        for right in 0..fruits.len() {
            *cnt.entry(fruits[right]).or_insert(0) += 1;
            while cnt.len() > 2 {
                let out = fruits[left];
                *cnt.get_mut(&out).unwrap() -= 1;
                if cnt[&out] == 0 {
                    cnt.remove(&out);
                }
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
