package leetcode

func getMinDistance(nums []int, target int, start int) int {
	if nums[start] == target {
		return 0
	}

	left, right := start-1, start+1

	n := len(nums)

	for left >= 0 || right < n {
		if left >= 0 && nums[left] == target {
			return start - left
		}
		if right < n && nums[right] == target {
			return right - start
		}
		left--
		right++
	}

	return 0
}
