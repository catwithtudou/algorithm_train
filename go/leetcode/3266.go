package leetcode

import (
	"container/heap"
	"sort"
)

func getFinalStateII(nums []int, k int, multiplier int) []int {
	if multiplier == 1 {
		return nums
	}

	n := len(nums)
	h := make(sHp, n)
	mx := 0
	for i, x := range nums {
		h[i] = sPair{x, i}
		mx = max(mx, x)
	}
	heap.Init(&h)

	for ; k > 0 && h[0].x < mx; k-- {
		h[0].x *= multiplier
		heap.Fix(&h, 0)
	}

	sort.Slice(h, func(i, j int) bool {
		return less(h[i], h[j])
	})
	for i, p := range h {
		e := k / n
		if i < k%n {
			e++
		}
		nums[p.i] = p.x % mod * calQuickPow(multiplier, e) % mod
	}

	return nums
}
