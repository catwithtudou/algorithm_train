package leetcode

func sortedSquares(nums []int) []int {
	k := len(nums) - 1
	result := make([]int, k+1)
	for i, j := 0, k; i <= j; {
		if nums[i]*nums[i] < nums[j]*nums[j] {
			result[k] = nums[j] * nums[j]
			k--
			j--
		} else {
			result[k] = nums[i] * nums[i]
			k--
			i++
		}
	}

	return result
}
