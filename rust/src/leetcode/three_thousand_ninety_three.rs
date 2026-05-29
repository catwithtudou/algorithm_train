pub struct Solution;

#[derive(Default)]
struct TrieNode {
    son: [Option<Box<TrieNode>>; 26],
    min_len: i32,
    best_index: i32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            son: Default::default(),
            min_len: i32::MAX,
            best_index: -1,
        }
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut root = TrieNode::new();

        for (i, word) in words_container.iter().enumerate() {
            let l = word.len() as i32;
            let bytes = word.as_bytes();

            // 更新根节点（对应没有共同后缀时的最长公共前缀，即空后缀）
            if l < root.min_len {
                root.min_len = l;
                root.best_index = i as i32;
            }

            let mut cur = &mut root;
            // 逆序遍历单词字节
            for &b in bytes.iter().rev() {
                let idx = (b - b'a') as usize;

                // 如果子节点不存在，则创建它
                if cur.son[idx].is_none() {
                    cur.son[idx] = Some(Box::new(TrieNode::new()));
                }
                // 移动到子节点
                cur = cur.son[idx].as_mut().unwrap();

                // 更新当前节点的最短长度和索引
                if l < cur.min_len {
                    cur.min_len = l;
                    cur.best_index = i as i32;
                }
            }
        }

        let mut ans = Vec::with_capacity(words_query.len());
        for word in words_query {
            let bytes = word.as_bytes();
            let mut cur = &root;

            // 逆序匹配后缀
            for &b in bytes.iter().rev() {
                let idx = (b - b'a') as usize;
                if let Some(next_node) = &cur.son[idx] {
                    cur = next_node;
                } else {
                    break;
                }
            }
            ans.push(cur.best_index);
        }

        ans
    }
}