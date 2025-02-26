pub struct Solution;

#[allow(dead_code)]
struct BrowserHistory {
    stack: Vec<String>,
    cur: usize,
}

#[allow(dead_code)]
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            stack: vec![homepage],
            cur: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.cur += 1;
        self.stack.truncate(self.cur);
        self.stack.push(url);
    }

    fn back(&mut self, steps: i32) -> String {
        self.cur = self.cur.saturating_sub(steps as usize);
        self.stack[self.cur].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.cur = (self.cur + steps as usize).min(self.stack.len() - 1);
        self.stack[self.cur].clone()
    }
}
