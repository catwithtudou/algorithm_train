pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    // 定义常量作为结构体的关联常量
    const DRAW: i32 = 0;
    const MOUSE_TURN: usize = 1;
    const CAT_TURN: usize = 2;
    const MOUSE_WIN: i32 = 1;
    const CAT_WIN: i32 = 2;

    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        // 使用 Self:: 访问关联常量
        let mut color = vec![vec![vec![Self::DRAW; 3]; n]; n];
        let mut degree = vec![vec![vec![0; 3]; n]; n];

        // 填充 degree 数组
        for m in 0..n {
            for c in 0..n {
                degree[m][c][Self::MOUSE_TURN] = graph[m].len();
                let cat_degree = graph[c].iter().filter(|&&x| x != 0).count();
                degree[m][c][Self::CAT_TURN] = cat_degree;
            }
        }

        let mut queue = VecDeque::new();

        // 初始化已知胜利状态
        for m in 0..n {
            for c in 0..n {
                if m == 0 {
                    color[m][c][Self::MOUSE_TURN] = Self::MOUSE_WIN;
                    color[m][c][Self::CAT_TURN] = Self::MOUSE_WIN;
                    queue.push_back((m, c, Self::MOUSE_TURN, Self::MOUSE_WIN));
                    queue.push_back((m, c, Self::CAT_TURN, Self::MOUSE_WIN));
                } else if m == c {
                    color[m][c][Self::MOUSE_TURN] = Self::CAT_WIN;
                    color[m][c][Self::CAT_TURN] = Self::CAT_WIN;
                    queue.push_back((m, c, Self::MOUSE_TURN, Self::CAT_WIN));
                    queue.push_back((m, c, Self::CAT_TURN, Self::CAT_WIN));
                }
            }
        }

        // BFS 处理队列
        while let Some((m, c, t, res)) = queue.pop_front() {
            let prev_states = Self::get_prev_states(&graph, m, c, t);
            for (prev_m, prev_c, prev_t) in prev_states {
                if color[prev_m][prev_c][prev_t] != Self::DRAW {
                    continue;
                }

                if (prev_t == Self::MOUSE_TURN && res == Self::MOUSE_WIN) ||
                   (prev_t == Self::CAT_TURN && res == Self::CAT_WIN)
                {
                    color[prev_m][prev_c][prev_t] = res;
                    queue.push_back((prev_m, prev_c, prev_t, res));
                } else {
                    degree[prev_m][prev_c][prev_t] -= 1;
                    if degree[prev_m][prev_c][prev_t] == 0 {
                        let lose_res = if prev_t == Self::CAT_TURN {
                            Self::MOUSE_WIN
                        } else {
                            Self::CAT_WIN
                        };
                        color[prev_m][prev_c][prev_t] = lose_res;
                        queue.push_back((prev_m, prev_c, prev_t, lose_res));
                    }
                }
            }
        }

        color[1][2][Self::MOUSE_TURN]
    }

    // 将辅助函数移动到 impl 块内
    fn get_prev_states(graph: &[Vec<i32>], m: usize, c: usize, t: usize) -> Vec<(usize, usize, usize)> {
        let mut prev_states = Vec::new();
        if t == Self::CAT_TURN {
            for &prev_m in &graph[m] {
                prev_states.push((prev_m as usize, c, Self::MOUSE_TURN));
            }
        } else {
            for &prev_c in &graph[c] {
                if prev_c != 0 {
                    prev_states.push((m, prev_c as usize, Self::CAT_TURN));
                }
            }
        }
        prev_states
    }
}