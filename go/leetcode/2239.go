package leetcode

func findClosestNumber(nums []int) int {
	dis, idx := -1, 0
	for i, x := range nums {
		if dis == -1 || dis > abs(x) {
			dis = abs(x)
			idx = i
		} else if dis == abs(x) {
			if nums[idx] < x {
				idx = i
			}
		}

	}

	return nums[idx]
}
