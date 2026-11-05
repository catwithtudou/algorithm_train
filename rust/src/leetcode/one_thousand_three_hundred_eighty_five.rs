pub struct Solution;


impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>,mut arr2: Vec<i32>, d: i32) -> i32 {
        arr2.sort_unstable();
        arr1.into_iter().filter(|&x| {
            let i = arr2.partition_point(|&y| y<x-d);
            i==arr2.len() || arr2[i] > x +d
        }).count() as i32
    }
}