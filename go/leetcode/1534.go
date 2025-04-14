package leetcode

import "slices"

func countGoodTriplets(arr []int, a int, b int, c int) (ans int) {
	mx := slices.Max(arr)
	s := make([]int, mx+2)
	for j, y := range arr {
		for _, z := range arr[j+1:] {
			if abs(y-z) > b {
				continue
			}
			l := max(max(y-a, z-c), 0)
			r := min(min(y+a, z+c), mx)
			ans += max(s[r+1]-s[l], 0)
		}
		for v := y + 1; v < mx+2; v++ {
			s[v]++
		}
	}

	return
}
