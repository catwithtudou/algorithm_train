pub struct Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rod: [i32; 10] = [0; 10];
        let cs: Vec<char> = rings.chars().collect();
        for i in (0..cs.len()).step_by(2) {
            let char = cs[i];
            let j = cs[i + 1] as usize - '0' as usize;
            match char {
                'G' => rod[j] |= 1,
                'B' => rod[j] |= 2,
                'R' => rod[j] |= 4,
                _ => {}
            }
        }

        rod.iter().filter(|&&x| x == 7).count() as i32
    }
}

#[cfg(test)]
mod two_thousand_one_hundred_and_three_test {
    use super::*;

    #[test]
    fn two_thousand_one_hundred_and_three() {
        assert_eq!(Solution::count_points(String::from("B0B6G0R6R0R6G9")), 1);
    }
}


