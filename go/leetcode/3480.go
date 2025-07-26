package leetcode

import "slices"

func maxSubarrays(n int, conflictingPairs [][]int) int64 {
	group := make([][]int, n+1)
	for _, p := range conflictingPairs {
		a, b := p[0], p[1]
		if a > b {
			a, b = b, a
		}
		group[a] = append(group[a], b)
	}

	extra := make([]int, n+2)
	b := []int{n + 1, n + 1}
	ans := 0

	for i := n; i > 0; i-- {
		b = append(b, group[i]...)
		slices.Sort(b)
		b = b[:2]
		ans += b[0] - i
		extra[b[0]] += b[1] - b[0]
	}

	return int64(ans + slices.Max(extra))
}
