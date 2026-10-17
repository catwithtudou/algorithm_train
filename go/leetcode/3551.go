package leetcode

import (
	"cmp"

	"golang.org/x/exp/slices"
)

func minSwaps3551(nums []int) int {
	n := len(nums)
	type tuple struct{ s, x, i int }
	a := make([]tuple, n)
	for i, num := range nums {
		s := 0
		for x := num; x > 0; x /= 10 {
			s += x % 10
		}
		a[i] = tuple{s, num, i}
	}

	slices.SortFunc(a, func(a, b tuple) int { return cmp.Or(a.s-b.s, a.x-b.x) })

	cc := 0

	for _, p := range a {
		if p.i < 0 {
			continue
		}
		cc++
		for i := p.i; i >= 0; {
			i, a[i].i = a[i].i, -1
		}
	}

	return n - cc
}
