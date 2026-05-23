package leetcode

func check(nums []int) bool {

	n := len(nums)

	sorted := true

	for i := 1; i < n; i++ {
		if nums[i-1] > nums[i] {
			if !sorted {
				return false
			}
			sorted = false
		}
	}

	return sorted || nums[0] >= nums[n-1]
}
