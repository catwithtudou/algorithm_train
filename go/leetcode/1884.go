package leetcode

import "math"

var memo [1001]int

func twoEggDrop(n int) int {
	if n == 0 {
		return 0
	}
	p := &memo[n]
	if *p != 0 {
		return *p
	}

	res := math.MaxInt
	for i := 1; i <= n; i++ {
		res = min(res, max(i, twoEggDrop(n-i)+1))
	}
	*p = res

	return res
}
