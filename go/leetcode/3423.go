package leetcode

func maxAdjacentDistance(nums []int) (ans int) {
	n := len(nums)
	for i := 1; i <= n; i++ {
		ans = max(ans, abs(nums[i%n]-nums[(i-1)%n]))
	}
	return
}
