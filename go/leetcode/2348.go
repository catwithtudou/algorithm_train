package leetcode

func zeroFilledSubarray(nums []int) (ans int64) {
	n := len(nums)
	preSum := 0
	for i := n - 1; i >= 0; i-- {
		if nums[i] == 0 {
			preSum++
		} else {
			preSum = 0
		}
		ans += int64(preSum)
	}

	return ans
}
