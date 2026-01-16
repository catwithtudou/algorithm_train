package leetcode

import (
	"golang.org/x/exp/slices"
)

func areaF(a []int, mx int) map[int]bool {
	a = append(a, 1, mx)
	area := map[int]bool{}
	slices.Sort(a)

	for i, x := range a {
		for _, y := range a[i+1:] {
			area[y-x] = true
		}
	}

	return area
}

func maximizeSquareArea(m int, n int, hFences []int, vFences []int) (ans int) {
	ha := areaF(hFences, m)
	va := areaF(vFences, n)
	for x := range ha {
		if va[x] {
			ans = max(ans, x)
		}
	}
	if ans == 0 {
		return -1
	}
	return ans * ans % (1e9 + 7)
}
