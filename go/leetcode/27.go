package leetcode

func removeElement(nums []int, val int) int {
	slow := 0
	for fast := 0; fast < len(nums); fast++ {
		if val != nums[fast] {
			nums[slow] = nums[fast]
			slow++
		}
	}
	return slow
}

func removeElementOther(nums []int, val int) int {
	leftIdx, rightIdx := 0, len(nums)-1
	for leftIdx <= rightIdx {

		for leftIdx <= rightIdx && nums[leftIdx] != val {
			leftIdx++
		}

		for leftIdx <= rightIdx && nums[rightIdx] == val {
			rightIdx--
		}

		if leftIdx < rightIdx {
			nums[leftIdx] = nums[rightIdx]
			leftIdx++
			rightIdx--
		}
	}

	return leftIdx
}
