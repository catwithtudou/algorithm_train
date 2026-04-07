pub struct Solution;

struct Robot {
    w: i32,
    h: i32,
    s: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {

    fn new(width: i32, height: i32) -> Self {
        Self {
            w: width,
            h: height,
            s: 0,
        }
    }

    fn step(&mut self, num: i32) {
        self.s = (self.s + num - 1) % ((self.w + self.h - 2) * 2) + 1;
    }

    fn get_state(&self) -> (i32,i32,String) {
        let w = self.w;
        let h = self.h;
        let s = self.s;
        if s < w {
            (s,0,"East".to_string())
        }else if s<w+h-1 {
            (w-1,s-w+1,"North".to_string())
        }else if s<w*2+h-2 {
            (w*2+h-s-3,h-1,"West".to_string())
        }else {
            (0,(w+h)*2-s-4,"South".to_string())
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        let (x, y, _) = self.get_state();
        vec![x, y]
    }

    fn get_dir(&self) -> String {
        let (_, _, dir) = self.get_state();
        dir
    }
}

