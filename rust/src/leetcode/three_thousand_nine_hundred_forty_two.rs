pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut cnt = 0;
        let mut pos = 0;

        // 检查是否只有一个下降位置
        for i in 1..n {
            if cnt >= 2 {
                break;
            }

            if nums[i - 1] > nums[i] {
                cnt += 1;
                pos = i;
            }
        }

        if cnt == 0 {
            return 0;
        }

        let mut ans = usize::MAX;

        if cnt == 1 && nums[0] > nums[n - 1] {
            ans = pos.min(n - pos + 2);
        }

        cnt = 0;
        pos = 0;

        // 检查是否只有一个上升位置
        for i in 1..n {
            if cnt >= 2 {
                break;
            }

            if nums[i - 1] < nums[i] {
                cnt += 1;
                pos = i;
            }
        }

        if cnt == 0 {
            return 1;
        }

        if cnt == 1 && nums[0] < nums[n - 1] {
            ans = ans.min((pos + 1).min(n - pos + 1));
        }

        if ans == usize::MAX {
            -1
        } else {
            ans as i32
        }
    }
}