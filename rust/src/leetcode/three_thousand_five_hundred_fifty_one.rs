pub struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // 数位和
        let digit_sum = |mut x: i32| {
            let mut s = 0;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            s
        };

        // idx[k] = 排好序后第 k 个位置应该放的原下标
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by_key(|&i| (digit_sum(nums[i]), nums[i]));

        // 并查集
        let mut fa: Vec<usize> = (0..n).collect();
        fn find(fa: &mut Vec<usize>, x: usize) -> usize {
            let mut root = x;
            while fa[root] != root {
                root = fa[root];
            }
            // 路径压缩
            let mut cur = x;
            while fa[cur] != root {
                let next = fa[cur];
                fa[cur] = root;
                cur = next;
            }
            root
        }

        // 每成功合并一次 = 一次交换
        let mut ans = 0;
        for k in 0..n {
            let (a, b) = (find(&mut fa, k), find(&mut fa, idx[k]));
            if a != b {
                fa[a] = b;
                ans += 1;
            }
        }
        ans
    }
}