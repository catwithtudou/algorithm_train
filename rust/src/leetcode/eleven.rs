﻿pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        while left < right {
            let area = (right - left) as i32 * height[left].min(height[right]);
            ans = ans.max(area);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}
