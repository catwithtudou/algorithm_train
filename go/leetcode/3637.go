package leetcode

func isTrionic(nums []int) bool {
	n := len(nums)

	i := 1
	for i < n && nums[i-1] < nums[i] {
		i++
	}
	if i == 1 {
		return false
	}

	i0 := i
	for i < n && nums[i-1] > nums[i] {
		i++
	}
	if i == i0 || i == n {
		return false
	}

	for i < n && nums[i-1] < nums[i] {
		i++
	}

	return i == n
}
