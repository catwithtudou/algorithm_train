package leetcode

func alternatingSubarray(nums []int) int {
	ans := -1
	i, n := 0, len(nums)
	for i < n-1 {
		if nums[i+1]-nums[i] != 1 {
			i++
			continue
		}
		originIdx := i
		i += 2
		for i < n && nums[i] == nums[i-2] {
			i++
		}
		ans = max(ans, i-originIdx)
		i--
	}

	return ans
}
