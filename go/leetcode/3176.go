package leetcode

func maximumLength3176(nums []int, k int) int {
	dp := make(map[int][]int, len(nums))
	mx := make([]int, k+2)
	for _, v := range nums {
		if dp[v] == nil {
			dp[v] = make([]int, k+1)
		}
		f := dp[v]
		for j := k; j >= 0; j-- {
			f[j] = max(f[j], mx[j]) + 1
			mx[j+1] = max(mx[j+1], f[j])
		}
	}
	return mx[k+1]
}
