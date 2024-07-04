package leetcode

import "math"

func minimumMoves(nums []int, k int, maxChanges int) int64 {
	pos := []int{}
	c := 0
	for i, v := range nums {
		if v == 0 {
			continue
		}
		pos = append(pos, i)
		c = max(c, 1)
		if i > 0 && nums[i-1] == 1 {
			if i > 1 && nums[i-2] == 1 {
				c = 3
			} else {
				c = max(c, 2)
			}
		}
	}
	c = min(c, k)
	if maxChanges >= k-c {
		return int64(max(c-1, 0) + (k-c)*2)
	}

	n := len(pos)
	sum := make([]int, n+1)
	for i, x := range pos {
		sum[i+1] = sum[i] + x
	}

	ans := math.MaxInt
	size := k - maxChanges
	for right := size; right <= n; right++ {
		left := right - size
		i := left + size/2
		s1 := pos[i]*(i-left) - (sum[i] - sum[left])
		s2 := sum[right] - sum[i] - pos[i]*(right-i)
		ans = min(ans, s1+s2)
	}
	return int64(ans + maxChanges*2)
}
