pub struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let n = s.len();
        let mut arr: Vec<u8> = s.bytes().collect();

        for i in 1..=n-2 {
            for j in 0..=n-1-i {
                let di1 = (arr[j]-b'0') as u8;
                let di2 = (arr[j+1]-b'0') as u8;
                arr[j] = ((di1+di2)%10) + b'0';
            }
        }

        arr[0] == arr[1]
    }
}