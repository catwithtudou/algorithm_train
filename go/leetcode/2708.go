package leetcode

func maxStrength(nums []int) int64 {
	mx, mn := nums[0], nums[0]
	for i := 1; i < len(nums); i++ {
		mn, mx = min(mn, min(min(nums[i], mn*nums[i]), mx*nums[i])), max(mx, max(max(nums[i], mx*nums[i]), mn*nums[i]))
	}
	return int64(mx)
}
