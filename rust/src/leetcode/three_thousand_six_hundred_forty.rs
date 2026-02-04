pub struct Solution;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut ans = i64::MIN;
        let mut i: usize = 0;

        while i < n {
            let start = i;

            i += 1;
            while i < n && nums[i - 1] < nums[i] {
                i += 1;
            }
            if i == start + 1 {
                continue;
            }

            let peak = i - 1;

            let mut res = nums[peak] as i64 + nums[peak - 1] as i64;

            while i < n && nums[i - 1] > nums[i] {
                res += nums[i] as i64;
                i += 1;
            }

            if i == peak + 1 || i == n || nums[i - 1] == nums[i] {
                continue;
            }

            let bottom = i - 1;

            // 加上 bottom 后面的第一个点（也就是下一段上升的起点）
            res += nums[i] as i64;

            // 向右找“上升段”的最大前缀和
            let mut max_s: i64 = 0;
            let mut s: i64 = 0;
            i += 1;
            while i < n && nums[i - 1] < nums[i] {
                s += nums[i] as i64;
                if s > max_s {
                    max_s = s;
                }
                i += 1;
            }
            res += max_s;

            // 向左找 peak 左侧（不含 peak-1, peak）的最大后缀和：j = peak-2..start
            max_s = 0;
            s = 0;
            for j in (start..peak - 1).rev() {
                s += nums[j] as i64;
                if s > max_s {
                    max_s = s;
                }
            }
            res += max_s;

            ans = ans.max(res);

            i = bottom;
        }

        ans
    }
}
