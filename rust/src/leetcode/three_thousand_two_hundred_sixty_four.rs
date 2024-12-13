use std::collections::BinaryHeap;
use std::cmp::Reverse;

const MOD: i64 = 1_000_000_007;

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    x: i64,
    i: usize,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Equal => self.i.cmp(&other.i),
            ord => ord,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }

        let n = nums.len();
        let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let multiplier = multiplier as i64;
        let k = k as i64;

        // 创建最小堆
        let mut heap: BinaryHeap<Reverse<Pair>> = BinaryHeap::new();
        let mut mx = 0i64;

        for (i, &x) in nums.iter().enumerate() {
            heap.push(Reverse(Pair { x, i }));
            mx = mx.max(x);
        }

        // 处理前k个最小值
        let mut k_remaining = k;
        while k_remaining > 0 && heap.peek().map_or(false, |p| p.0.x < mx) {
            if let Some(Reverse(mut pair)) = heap.pop() {
                // 修改这里的乘法和取余操作
                pair.x = Self::mul_mod(pair.x, multiplier);
                heap.push(Reverse(pair));
                k_remaining -= 1;
            }
        }

        // 将堆转换为有序数组
        let mut pairs: Vec<Pair> = heap.into_iter().map(|r| r.0).collect();
        pairs.sort_by(|a, b| {
            if a.x != b.x {
                a.x.cmp(&b.x)
            } else {
                a.i.cmp(&b.i)
            }
        });

        // 计算最终结果
        for (i, pair) in pairs.iter().enumerate() {
            let mut e = k_remaining / (n as i64);
            if i < (k_remaining % (n as i64)) as usize {
                e += 1;
            }
            // 修改这里的乘法和取余操作
            nums[pair.i] = Self::mul_mod(pair.x, Self::quick_pow(multiplier, e));
        }

        nums.into_iter().map(|x| x as i32).collect()
    }

    // 安全的模乘运算
    fn mul_mod(a: i64, b: i64) -> i64 {
        let a = a % MOD;
        let b = b % MOD;
        ((a as i128 * b as i128) % MOD as i128) as i64
    }

    // 修改后的快速幂实现
    fn quick_pow(mut x: i64, mut n: i64) -> i64 {
        x = x % MOD;
        let mut res = 1i64;
        while n > 0 {
            if n % 2 > 0 {
                res = Self::mul_mod(res, x);
            }
            x = Self::mul_mod(x, x);
            n /= 2;
        }
        res
    }
}