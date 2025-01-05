package leetcode

var atmDenominations = [...]int{20, 50, 100, 200, 500}

const atmKinds = len(atmDenominations)

type ATM [atmKinds]int

func ConstructorATM() ATM {
	return ATM{}
}

func (this *ATM) Deposit(banknotesCount []int) {
	for i, count := range banknotesCount {
		this[i] += count
	}
}

func (this *ATM) Withdraw(amount int) []int {
	ans := make([]int, atmKinds)

	for i := atmKinds - 1; i >= 0; i-- {
		ans[i] = min(amount/atmDenominations[i], this[i])
		amount -= ans[i] * atmDenominations[i]
	}

	if amount > 0 {
		return []int{-1}
	}

	for i, count := range ans {
		this[i] -= count
	}

	return ans
}

/**
 * Your ATM object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Deposit(banknotesCount);
 * param_2 := obj.Withdraw(amount);
 */
