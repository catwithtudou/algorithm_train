package leetcode

import (
	"math"
	"sort"
)

func maxPower(stations []int, r int, k int) int64 {
	n := len(stations)
	sum := make([]int, n+1)
	for i, x := range stations {
		sum[i+1] = sum[i] + x
	}

	power := make([]int, n)
	mn := math.MaxInt
	for i := range power {
		power[i] = sum[min(i+r+1, n)] - sum[max(i-r, 0)]
		mn = min(mn, power[i])
	}

	left := mn + k/n
	right := mn + k
	ans := left + sort.Search(right-left, func(low int) bool {
		low += left + 1
		diff := make([]int, n+1)
		sumD, built := 0, 0
		for i, p := range power {
			sumD += diff[i]
			m := low - (p + sumD)
			if m <= 0 {
				continue
			}
			built += m
			if built > k {
				return true
			}
			sumD += m
			diff[min(i+r*2+1, n)] -= m
		}
		return false
	})

	return int64(ans)
}
