package leetcode

func hasIncreasingSubarrays(nums []int, k int) bool {
	for i := 0; i <= len(nums)-2*k; i++ {
		has := true
		for j := 0; j < k-1; j++ {
			if nums[i+j] >= nums[i+j+1] || nums[i+j+k] >= nums[i+j+k+1] {
				has = false
				break
			}
		}
		if has {
			return true
		}
	}
	return false
}
