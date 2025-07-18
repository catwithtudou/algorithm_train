pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let m = nums.len();
        let n = m / 3;

        // 创建最小堆来维护后n个最大元素
        let mut min_heap: BinaryHeap<Reverse<i32>> =
            nums[m - n..].iter().map(|&x| Reverse(x)).collect();

        let mut sum: i64 = nums[m - n..].iter().map(|&x| x as i64).sum();

        let mut suf_max = vec![0i64; m - n + 1];
        suf_max[m - n] = sum;

        for i in (n..m - n).rev() {
            let v = nums[i];
            if let Some(&Reverse(min_val)) = min_heap.peek() {
                if v > min_val {
                    sum += (v - min_val) as i64;
                    min_heap.pop();
                    min_heap.push(Reverse(v));
                }
            }
            suf_max[i] = sum;
        }

        // 创建最大堆来维护前n个最小元素
        let mut max_heap: BinaryHeap<i32> = nums[..n].iter().cloned().collect();

        let mut pre_min: i64 = nums[..n].iter().map(|&x| x as i64).sum();

        let mut ans = pre_min - suf_max[n];

        for i in n..m - n {
            let v = nums[i];
            if let Some(&max_val) = max_heap.peek() {
                if v < max_val {
                    pre_min += (v - max_val) as i64;
                    max_heap.pop();
                    max_heap.push(v);
                }
            }
            ans = ans.min(pre_min - suf_max[i + 1]);
        }

        ans
    }
}
