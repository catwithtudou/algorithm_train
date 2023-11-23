pub struct Solution;

impl Solution {
    pub  fn entity_parser(text: String) -> String {
        let entities = [
            ("&quot;".to_string().into_bytes(), b'"'),
            ("&apos;".to_string().into_bytes(), b'\''),
            ("&amp;".to_string().into_bytes(), b'&'),
            ("&gt;".to_string().into_bytes(), b'>'),
            ("&lt;".to_string().into_bytes(), b'<'),
            ("&frasl;".to_string().into_bytes(), b'/'),
        ];

        let text = text.into_bytes();
        let mut ans = Vec::with_capacity(text.len());
        let mut i = 0;
        while i<text.len(){
            let mut flag = false;
            if text[i] == b'&' {
                for t in entities.iter(){
                    let len = t.0.len();
                    if i+len<=text.len() && text[i..i+len]==t.0{
                        ans.push(t.1);
                        i+=len;
                        flag = true;
                        break;
                    }
                }
            }

            if !flag {
                ans.push(text[i]);
                i += 1;
            }
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}