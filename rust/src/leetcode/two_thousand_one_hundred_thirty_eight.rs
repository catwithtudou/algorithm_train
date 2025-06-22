pub struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        s.into_bytes()
            .chunks(k)
            .map(|chunk| {
                let mut t = chunk.to_vec();
                if t.len() < k {
                    t.extend(std::iter::repeat(fill as u8).take(k - t.len()));
                }
                unsafe { String::from_utf8_unchecked(t) }
            })
            .collect()
    }
}
