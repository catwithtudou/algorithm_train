package leetcode

import "math"

func maximumEnergy(energy []int, k int) int {
	n := len(energy)
	ans := math.MinInt
	for i := n - k; i < n; i++ {
		curSum := 0
		for j := i; j >= 0; j -= k {
			curSum += energy[j]
			ans = max(ans, curSum)
		}
	}
	return ans
}
