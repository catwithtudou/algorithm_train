package leetcode

import "slices"

var pow2 = [1000_000]int{1}

func init1498() {
	for i := 1; i < len(pow2); i++ {
		pow2[i] = pow2[i-1] * 2 % mod
	}
}

func numSubseq(nums []int, target int) (ans int) {
	left, right := 0, len(nums)-1
	slices.Sort(nums)

	for left <= right {
		if nums[left]+nums[right] <= target {
			ans += pow2[right-left]
			left++
		} else {
			right--
		}
	}

	return ans % mod
}
