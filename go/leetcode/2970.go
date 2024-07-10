package leetcode

func incremovableSubarrayCount(nums []int) int {
	n := len(nums)
	i := 0
	for i < n-1 && nums[i] < nums[i+1] {
		i++
	}
	if i == n-1 {
		return n * (n + 1) / 2
	}
	ans := i + 2
	for j := n - 1; j == n-1 || nums[j] < nums[j+1]; j-- {
		for i >= 0 && nums[i] >= nums[j] {
			i--
		}
		ans += i + 2
	}
	return ans
}
