pub struct Solution;

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        #[derive(Default)]
        struct CharacterTrie {
            children: [Option<Box<CharacterTrie>>; 26],
        }

        impl CharacterTrie {
            fn new() -> Self {
                Self {
                    children: Default::default(),
                }
            }

            fn insert(&mut self, word: &str) {
                let mut node = self;
                for &c in word.as_bytes() {
                    let idx = (c - b'a') as usize;
                    node = node.children[idx].get_or_insert_with(|| Box::new(CharacterTrie::new()));
                }
            }
        }

        let mut trie = CharacterTrie::new();
        for word in words {
            trie.insert(&word);
        }

        let inf = 1 << 30;
        let n = target.len();
        let mut memo = vec![0; n];
        let target = target.as_bytes();

        fn dfs(
            i: usize,
            target: &[u8],
            trie: &CharacterTrie,
            memo: &mut Vec<i32>,
            inf: i32,
        ) -> i32 {
            if i >= target.len() {
                return 0;
            }

            if memo[i] != 0 {
                return memo[i];
            }

            let mut node = trie;
            memo[i] = inf;

            for j in i..target.len() {
                let idx = (target[j] - b'a') as usize;
                if node.children[idx].is_none() {
                    break;
                }
                memo[i] = memo[i].min(dfs(j + 1, target, trie, memo, inf) + 1);
                node = node.children[idx].as_ref().unwrap();
            }

            memo[i]
        }

        let ans = dfs(0, target, &trie, &mut memo, inf);
        if ans < inf {
            ans
        } else {
            -1
        }
    }
}
