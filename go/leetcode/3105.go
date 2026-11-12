package leetcode

func longestMonotonicSubarray(nums []int) (ans int) {
	n := len(nums)
	ans = 1
	i := 0

	for i < n-1 {
		if nums[i] == nums[i+1] {
			i++
			continue
		}
		inc := nums[i] < nums[i+1]
		start := i
		i += 2
		for i < n && nums[i] != nums[i-1] && nums[i] > nums[i-1] == inc {
			i++
		}
		ans = max(ans, i-start)
		i--
	}

	return
}
