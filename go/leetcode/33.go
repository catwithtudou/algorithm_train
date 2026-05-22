package leetcode

func search33(nums []int, target int) int {
	last := nums[len(nums)-1]
	left, right := -1, len(nums)-1

	for left+1 < right {
		mid := left + (right-left)/2
		x := nums[mid]
		if target > last && x <= last {
			right = mid
		} else if x > last && target <= last {
			left = mid
		} else if x >= target {
			right = mid
		} else {
			left = mid
		}
	}

	if nums[right] != target {
		return -1
	}

	return right
}
