package leetcode

func minimumOperations3396(nums []int) int {
	n := len(nums)
	cnt := make([]int, 101)
	for i := n - 1; i >= 0; i-- {
		if cnt[nums[i]] > 0 {
			return i/3 + 1
		}
		cnt[nums[i]]++
	}

	return 0
}
