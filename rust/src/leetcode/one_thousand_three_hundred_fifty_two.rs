pub struct Solution;


struct ProductOfNumbers {
    prefix_product: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            prefix_product: vec![1],
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix_product.clear();
            self.prefix_product.push(1);
        } else {
            let last = *self.prefix_product.last().unwrap();
            self.prefix_product.push(num * last);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let n = self.prefix_product.len();
        let k = k as usize;
        if n <= k {
            return 0;
        }
        self.prefix_product[n - 1] / self.prefix_product[n - 1 - k]
    }
}

