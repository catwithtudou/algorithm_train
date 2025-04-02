package leetcode

func maximumTripletValue(nums []int) int64 {
	n := len(nums)
	sufMax := make([]int, n+1)
	for i := n - 1; i > 1; i-- {
		sufMax[i] = max(sufMax[i+1], nums[i])
	}

	preMax, ans := 0, 0
	for i, x := range nums {
		ans = max(ans, (preMax-x)*sufMax[i+1])
		preMax = max(preMax, x)
	}

	return int64(ans)
}
