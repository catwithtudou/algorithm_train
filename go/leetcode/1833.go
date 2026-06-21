package leetcode

import "slices"

func maxIceCream(costs []int, coins int) int {
	slices.Sort(costs)

	for i, x := range costs {
		if x > coins {
			return i
		}
		coins -= x
	}

	return len(costs)
}
