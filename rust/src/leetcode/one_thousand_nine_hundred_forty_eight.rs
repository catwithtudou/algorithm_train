pub struct Solution;

use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    son: Option<HashMap<String, Box<TrieNode>>>,
    name: String,
    deleted: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            son: None,
            name: String::new(),
            deleted: false,
        }
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut root = TrieNode::new();

        // 构建字典树
        for path in &paths {
            let mut cur = &mut root;
            for s in path {
                if cur.son.is_none() {
                    cur.son = Some(HashMap::new());
                }
                let son_map = cur.son.as_mut().unwrap();
                if !son_map.contains_key(s) {
                    son_map.insert(s.clone(), Box::new(TrieNode::new()));
                }
                cur = son_map.get_mut(s).unwrap();
                cur.name = s.clone();
            }
        }

        let mut expr_to_node: HashMap<String, *mut TrieNode> = HashMap::new();

        fn gen_expr(
            node: &mut TrieNode,
            expr_to_node: &mut HashMap<String, *mut TrieNode>,
        ) -> String {
            if node.son.is_none() {
                return node.name.clone();
            }

            let mut expr: Vec<String> = Vec::new();
            let son_map = node.son.as_mut().unwrap();
            for son in son_map.values_mut() {
                expr.push(format!("({})", gen_expr(son, expr_to_node)));
            }
            expr.sort();

            let sub_tree_expr = expr.join("");
            if let Some(&existing_node_ptr) = expr_to_node.get(&sub_tree_expr) {
                unsafe {
                    (*existing_node_ptr).deleted = true;
                }
                node.deleted = true;
            } else {
                expr_to_node.insert(sub_tree_expr.clone(), node as *mut TrieNode);
            }

            format!("{}{}", node.name, sub_tree_expr)
        }

        if let Some(ref mut son_map) = root.son {
            for son in son_map.values_mut() {
                gen_expr(son, &mut expr_to_node);
            }
        }

        let mut ans: Vec<Vec<String>> = Vec::new();
        let mut path: Vec<String> = Vec::new();

        fn dfs(node: &TrieNode, path: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
            if node.deleted {
                return;
            }
            path.push(node.name.clone());
            ans.push(path.clone());

            if let Some(ref son_map) = node.son {
                for son in son_map.values() {
                    dfs(son, path, ans);
                }
            }
            path.pop();
        }

        if let Some(ref son_map) = root.son {
            for son in son_map.values() {
                dfs(son, &mut path, &mut ans);
            }
        }

        ans
    }
}