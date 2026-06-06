package leetcode

func leftRightDifference(nums []int) []int {
	total := 0

	for _, x := range nums {
		total += x
	}

	leftSum := 0

	for i, x := range nums {
		nums[i] = abs(2*leftSum + x - total)
		leftSum += x
	}

	return nums
}
