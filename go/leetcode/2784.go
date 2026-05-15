package leetcode

func isGood(nums []int) bool {
	n := len(nums) - 1
	cnt := make([]int, n+1)
	for _, x := range nums {
		if x > n ||
			x == n && cnt[x] > 1 || x < n && cnt[x] > 0 {
			return false
		}
		cnt[x]++
	}
	return true
}
