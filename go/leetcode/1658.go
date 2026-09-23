package leetcode

func minOperations1658(nums []int, x int) int {
	target := -x
	for _, x := range nums {
		target += x
	}
	if target < 0 {
		return -1
	}

	ans, left, sum := -1, 0, 0
	for right, num := range nums {
		sum += num
		for sum > target {
			sum -= nums[left]
			left++
		}
		if sum == target {
			ans = max(ans, right-left+1)
		}
	}

	if ans < 0 {
		return -1
	}
	return len(nums) - ans
}
