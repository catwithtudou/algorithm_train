package leetcode

type ProductOfNumbers []int

func ProductOfNumbersConstructor() ProductOfNumbers {
	return []int{1}
}

func (p *ProductOfNumbers) Add(num int) {
	if num != 0 {
		*p = append(*p, (*p)[len(*p)-1]*num)
	} else {
		*p = []int{1}
	}
}

func (p *ProductOfNumbers) GetProduct(k int) int {
	n := len(*p)
	if n <= k {
		return 0
	}
	return (*p)[n-1] / (*p)[n-1-k]
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Add(num);
 * param_2 := obj.GetProduct(k);
 */
