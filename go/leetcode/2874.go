package leetcode

func maximumTripletValueII(nums []int) int64 {
	ans := 0
	preMax, maxDiff := 0, 0
	for _, x := range nums {
		ans = max(ans, maxDiff*x)
		maxDiff = max(maxDiff, preMax-x)
		preMax = max(preMax, x)
	}
	return int64(ans)
}
