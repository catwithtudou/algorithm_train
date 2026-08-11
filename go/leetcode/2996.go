package leetcode

func missingInteger(nums []int) int {
	sum := nums[0]
	for i := 1; i < len(nums) && nums[i] == nums[i-1]+1; i++ {
		sum += nums[i]
	}

	has := make(map[int]bool)
	for _, x := range nums {
		has[x] = true
	}
	for has[sum] {
		sum++
	}
	return sum
}
