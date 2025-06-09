pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let count_subtree_size = |node: i32| -> i32 {
            let n = n as i64;
            let mut size = 0;
            let (mut left, mut right) = (node as i64, node as i64 + 1);
            while left <= n {
                size += (right.min(n + 1) - left) as i32;
                left *= 10;
                right *= 10;
            }
            size
        };

        let mut node = 1;
        k -= 1;
        while k > 0 {
            let size = count_subtree_size(node);
            if size <= k {
                node += 1;
                k -= size;
            } else {
                node *= 10;
                k -= 1;
            }
        }
        node
    }
}
