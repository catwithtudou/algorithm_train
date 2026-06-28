pub struct Solution;

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let n = arr.len();
        arr[0] = 1;
        for i in 1..n {
            arr[i] = arr[i].min(arr[i - 1] + 1);
        }
        arr[n - 1]
    }
}
