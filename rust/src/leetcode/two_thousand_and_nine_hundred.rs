pub struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = groups.len();
        let mut ans = vec![];
        for (i, word) in words.into_iter().enumerate() {
            if i == n - 1 || groups[i] != groups[i + 1] {
                ans.push(word);
            }
        }
        ans
    }
}
