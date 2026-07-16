package leetcode

import "slices"

func gcdSum(nums []int) (ans int64) {
	n := len(nums)
	mx := 0
	pre := make([]int, n)
	for i, x := range nums {
		mx = max(mx, x)
		pre[i] = gcd(x, mx)
	}

	slices.Sort(pre)
	for i := range n / 2 {
		ans += int64(gcd(pre[i], pre[n-1-i]))
	}

	return
}
