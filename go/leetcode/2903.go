package leetcode

func findIndices(nums []int, indexDifference int, valueDifference int) []int {
	maxIdx, minIdx := 0, 0
	for j := indexDifference; j < len(nums); j++ {
		i := j - indexDifference
		if nums[i] > nums[maxIdx] {
			maxIdx = i
		} else if nums[i] < nums[minIdx] {
			minIdx = i
		}
		if nums[maxIdx]-nums[j] >= valueDifference {
			return []int{maxIdx, j}
		} else if nums[j]-nums[minIdx] >= valueDifference {
			return []int{minIdx, j}
		}

	}

	return []int{-1, -1}
}
