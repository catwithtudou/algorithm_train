pub struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut fa = [0u8; 26];
        for i in 0..26 {
            fa[i] = i as u8;
        }

        fn find(fa: &mut [u8; 26], x: u8) -> u8 {
            if fa[x as usize] != x {
                fa[x as usize] = find(fa, fa[x as usize]);
            }
            fa[x as usize]
        }

        let merge = |fa: &mut [u8; 26], x: u8, y: u8| {
            let (mut small, mut big) = (find(fa, x), find(fa, y));
            if small > big {
                std::mem::swap(&mut small, &mut big);
            }
            fa[big as usize] = small;
        };

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        for i in 0..s1_bytes.len() {
            merge(&mut fa, s1_bytes[i] - b'a', s2_bytes[i] - b'a');
        }

        let mut result = Vec::with_capacity(base_str.len());
        for byte in base_str.bytes() {
            result.push(find(&mut fa, byte - b'a') + b'a');
        }

        String::from_utf8(result).unwrap()
    }
}
