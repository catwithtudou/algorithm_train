package leetcode

func removeDuplicates(nums []int) int {
	stackSize := 2
	for i := 2; i < len(nums); i++ {
		if nums[i] != nums[stackSize-2] {
			nums[stackSize] = nums[i]
			stackSize++
		}
	}
	return min(stackSize, len(nums))
}
