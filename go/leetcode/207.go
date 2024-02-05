package leetcode

func minSubArrayLen(target int, nums []int) int {
	start, end := 0, 0
	result := len(nums) + 1
	cur := 0
	for ; end < len(nums); end++ {
		cur += nums[end]
		for ; cur >= target; start-- {
			result = min(result, end-start+1)
			cur -= nums[start]
		}
	}

	if result == len(nums)+1 {
		return 0
	}
	return result
}
