pub struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pairs = vec![];
        for (i, arr) in nums.iter().enumerate() {
            for &x in arr {
                pairs.push((x, i));
            }
        }
        pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let (mut ans_l, mut ans_r) = (pairs[0].0, pairs[pairs.len() - 1].0);
        let mut empty = nums.len();
        let mut cnt = vec![0; empty];
        let mut left = 0;
        for &(r, i) in &pairs {
            if cnt[i] == 0 {
                empty -= 1;
            }
            cnt[i] += 1;
            while empty == 0 {
                let (l, i) = pairs[left];
                if r - l < ans_r - ans_l {
                    ans_l = l;
                    ans_r = r;
                }
                cnt[i] -= 1;
                if cnt[i] == 0 {
                    empty += 1;
                }
                left += 1;
            }
        }
        vec![ans_l, ans_r]
    }
}
