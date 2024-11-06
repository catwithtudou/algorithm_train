package leetcode

func resultsArrayII(nums []int, k int) []int {
	n := len(nums)
	ans := make([]int, n-k+1)
	for i := range ans {
		ans[i] = -1
	}
	cnt := 0
	for i, num := range nums {
		if i == 0 || num == nums[i-1]+1 {
			cnt++
		} else {
			cnt = 1
		}
		if cnt >= k {
			ans[i-k+1] = num
		}
	}
	return ans
}
