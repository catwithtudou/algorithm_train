pub struct Solution;

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut pre = -1;
        for i in (0..arr.len()).rev() {
            let cur = arr[i];
            arr[i] = pre;
            pre = pre.max(cur);
        }
        arr
    }
}
