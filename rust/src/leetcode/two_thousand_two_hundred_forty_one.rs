pub struct Solution;

struct ATM {
    account: [i32; ATM::KINDS],
}

impl ATM {
    const DENOMINATIONS: [i32; 5] = [20, 50, 100, 200, 500];
    const KINDS: usize = Self::DENOMINATIONS.len();

    fn new() -> Self {
        Self {
            account: [0; Self::KINDS],
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for i in 0..Self::KINDS {
            self.account[i] += banknotes_count[i];
        }
    }

    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let mut ans = vec![0; Self::KINDS];

        for i in (0..Self::KINDS).rev() {
            ans[i] = self.account[i].min(amount / Self::DENOMINATIONS[i]);
            amount -= ans[i] * Self::DENOMINATIONS[i];
        }

        if amount > 0 {
            return vec![-1];
        }

        for i in 0..Self::KINDS {
            self.account[i] -= ans[i];
        }

        ans
    }
}
