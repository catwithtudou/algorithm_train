use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap();
        // 求最高比特位
        let high_bit = 31 - mx.leading_zeros() as i32;

        let mut ans = 0;
        let mut mask = 0;
        let mut visited = HashSet::new();

        for i in (0..=high_bit).rev() {
            visited.clear();
            mask |= 1 << i;
            let new_ans = ans | (1 << i);
            for &x in &nums {
                let x = x & mask;
                if visited.contains(&(new_ans ^ x)) {
                    ans = new_ans;
                    break;
                }
                visited.insert(x);
            }
        }

        ans
    }

    pub fn find_maximum_xor_trie(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        let mut ans = 0;
        for &x in nums.iter() {
            trie.add(x);
            ans = ans.max(trie.search(x))
        }
        ans
    }
}

struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Trie {
        Trie {
            children: [None, None],
        }
    }

    fn add(&mut self, x: i32) {
        let mut node = self;
        for i in (0..=30).rev() {
            let v = (x >> i & 1) as usize;
            if node.children[v].is_none() {
                node.children[v] = Some(Box::new(Trie::new()));
            }
            node = node.children[v].as_mut().unwrap();
        }
    }

    fn search(&self, x: i32) -> i32 {
        let mut node = self;
        let mut ans = 0;
        for i in (0..=30).rev() {
            let v = (x >> i & 1) as usize;
            if let Some(child) = &node.children[v ^ 1] {
                ans |= 1 << i;
                node = child.as_ref();
            } else {
                node = node.children[v].as_ref().unwrap();
            }
        }
        ans
    }
}


#[cfg(test)]
mod four_hundred_and_twenty_one_test {
    use super::*;

    #[test]
    fn four_hundred_and_twenty_one() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
        assert_eq!(Solution::find_maximum_xor_trie(vec![3, 10, 5, 25, 2, 8]), 28);
    }
}