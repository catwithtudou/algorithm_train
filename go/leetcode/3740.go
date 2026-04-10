package leetcode

import "math"

func minimumDistance3740(nums []int) int {
	pos := map[int][]int{}

	for i, x := range nums {
		pos[x] = append(pos[x], i)
	}

	ans := math.MaxInt
	for _, p := range pos {
		for i := 2; i < len(p); i++ {
			ans = min(ans, (p[i]-p[i-2])*2)
		}
	}

	if ans == math.MaxInt {
		return -1
	}

	return ans
}
