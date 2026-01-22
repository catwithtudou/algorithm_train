package leetcode

func minimumPairRemoval(nums []int) int {
	count := 0

	for len(nums) > 0 {
		isAscending := true
		minSum := 1<<31 - 1
		targetIdx := -1

		for i := 0; i < len(nums)-1; i++ {
			sum := nums[i] + nums[i+1]
			if nums[i] > nums[i+1] {
				isAscending = false
			}
			if sum < minSum {
				minSum = sum
				targetIdx = i
			}
		}

		if isAscending {
			break
		}

		count++
		nums[targetIdx] = minSum
		nums = append(nums[:targetIdx+1], nums[targetIdx+2:]...)
	}

	return count
}
