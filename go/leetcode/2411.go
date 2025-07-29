package leetcode

func smallestSubarrays(nums []int) []int {
	ans := make([]int, len(nums))

	for i, x := range nums {
		ans[i] = 1
		for j := i - 1; j >= 0 && nums[j]|x != nums[j]; j-- {
			nums[j] |= x
			ans[j] = i - j + 1
		}
	}

	return ans
}
