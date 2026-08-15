package leetcode

func longestSubsequence3702(nums []int) int {
	sum, xor := 0, 0
	for _, num := range nums {
		sum += num
		xor ^= num
	}

	if sum == 0 {
		return 0
	}

	ans := len(nums)
	if xor == 0 {
		ans--
	}

	return ans
}
