pub struct Solution;

impl Solution {
    pub fn ok(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = false;
        for (ca, cb) in a.chars().zip(b.chars()) {
            if ca != cb {
                if diff {
                    return false;
                }
                diff = true;
            }
        }
        diff
    }

    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = groups.len();
        let mut f = vec![0; n];
        let mut from = vec![0; n];
        let mut max_i = n - 1;
        for i in (0..n).rev() {
            for j in i + 1..n {
                if f[j] > f[i] && groups[i] != groups[j] && Self::ok(&words[i], &words[j]) {
                    f[i] = f[j];
                    from[i] = j;
                }
            }
            f[i] += 1;
            if f[i] > f[max_i] {
                max_i = i;
            }
        }

        let mut ans = Vec::with_capacity(f[max_i]);
        let mut k = max_i;

        for _ in 0..f[max_i] {
            ans.push(words[k].clone());
            k = from[k];
        }

        ans
    }
}
