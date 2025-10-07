pub struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while a != 0 {
                let tmp = a;
                a = b % a;
                b = tmp;
            }
            b
        }
        fn lcm(a: i32, b: i32) -> i32 {
            a / gcd(a, b) * b
        }

        let mut st = vec![];
        for mut x in nums {
            while !st.is_empty() && gcd(x, *st.last().unwrap()) > 1 {
                x = lcm(x, st.pop().unwrap());
            }
            st.push(x);
        }
        st
    }
}
