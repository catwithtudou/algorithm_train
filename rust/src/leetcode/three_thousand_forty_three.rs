pub struct Solution;

use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut hang = HashSet::new();
        let mut ans = 0;

        // 1. 将 arr1 中所有数字的所有前缀存入 HashSet
        for v in arr1 {
            let s = v.to_string();
            // 在 Rust 中，字符串切片是按字节索引的。因为这里全是数字（ASCII），
            // 所以可以直接用安全切片 &s[0..i]
            for i in 1..=s.len() {
                hang.insert(s[0..i].to_string());
            }
        }

        // 2. 遍历 arr2，匹配最长前缀
        for v in arr2 {
            let s = v.to_string();
            for i in 1..=s.len() {
                // 如果当前前缀存在于集合中，更新最大长度
                if hang.contains(&s[0..i]) {
                    ans = max(ans, i as i32);
                } else {
                    // 如果当前长度的前缀不匹配，更长的也不可能匹配，直接跳出内层循环
                    break;
                }
            }
        }

        ans
    }
}