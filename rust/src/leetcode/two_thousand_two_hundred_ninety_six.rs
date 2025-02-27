#[allow(dead_code)]
pub struct TextEditor {
    left: Vec<u8>,  // 光标左侧字符
    right: Vec<u8>, // 光标右侧字符
}

#[allow(dead_code)]
impl TextEditor {
    fn new() -> Self {
        TextEditor {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn add_text(&mut self, text: String) {
        self.left.extend(text.as_bytes());
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let delete_count = k.min(self.left.len() as i32);
        self.left.truncate(self.left.len() - delete_count as usize);
        delete_count
    }

    fn text(&self) -> String {
        let start = (self.left.len() as i32 - 10).max(0) as usize;
        String::from_utf8(self.left[start..].to_vec()).unwrap()
    }

    fn cursor_left(&mut self, mut k: i32) -> String {
        while k > 0 && !self.left.is_empty() {
            if let Some(byte) = self.left.pop() {
                self.right.push(byte);
            }
            k -= 1;
        }
        self.text()
    }

    fn cursor_right(&mut self,mut k: i32) -> String {
        while k > 0 && !self.right.is_empty() {
            if let Some(byte) = self.right.pop() {
                self.left.push(byte);
            }
            k -= 1;
        }
        self.text()
    }
}
