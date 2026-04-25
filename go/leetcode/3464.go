package leetcode

import (
	"slices"
	"sort"
)

func maxDistance3464(side int, points [][]int, k int) int {
	a := make([]int, len(points))
	for i, p := range points {
		x, y := p[0], p[1]
		if x == 0 {
			a[i] = y
		} else if y == side {
			a[i] = side + x
		} else if x == side {
			a[i] = 3*side - y
		} else {
			a[i] = 4*side - x
		}
	}

	slices.Sort(a)

	ans := sort.Search(side*4/k, func(low int) bool {
		low++

	next:
		for i, start := range a {
			cur := start
			for range k - 1 {
				i += sort.Search(len(a)-i, func(j int) bool { return a[i+j]-cur >= low })
				if i == len(a) || a[i] > start+side*4-low {
					continue next
				}
				cur = a[i]
			}
			return false
		}
		return true
	})

	return ans
}
