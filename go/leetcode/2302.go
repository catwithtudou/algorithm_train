package leetcode

func countSubarrays2302(nums []int, k int64) (ans int64) {
	left, preSum := 0, int64(0)
	for right, v := range nums {
		preSum += int64(v)
		for preSum*int64(right-left+1) >= k {
			preSum -= int64(nums[left])
			left++
		}
		ans += int64(right - left + 1)
	}
	return ans
}
