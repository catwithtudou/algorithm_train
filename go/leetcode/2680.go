package leetcode

func maximumOr(nums []int, k int) int64 {
	n := len(nums)

	suf := make([]int, n)

	for i := n - 2; i >= 0; i-- {
		suf[i] = suf[i+1] | nums[i+1]
	}

	pre, ans := 0, 0

	for i, x := range nums {
		ans = max(ans, pre|x<<k|suf[i])
		pre |= x
	}
	return int64(ans)
}
