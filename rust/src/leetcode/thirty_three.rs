pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let last = nums[n - 1];
        let mut left = 0;
        let mut right = n - 1; // 左闭右开区间 [0, n-1)
        while left < right { // 区间不为空
            let mid = left + (right - left) / 2;
            let x = nums[mid];
            if target > last && x <= last { // target 在第一段，x 在第二段
                right = mid; // 下轮循环去左边找
            } else if x > last && target <= last { // x 在第一段，target 在第二段
                left = mid + 1; // 下轮循环去右边找
            } else if x >= target { // 否则，x 和 target 在同一段，这就和方法一的 lower_bound 一样了
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if nums[left] == target { left as _ } else { -1 }
    }
}