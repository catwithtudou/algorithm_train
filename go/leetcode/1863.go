package leetcode

func subsetXORSum(nums []int) int {
	var dfs func(int, int) int
	dfs = func(idx int, xor int) int {
		if idx == len(nums) {
			return xor
		}
		return dfs(idx+1, xor^nums[idx]) + dfs(idx+1, xor)
	}
	return dfs(0, 0)
}

func subsetXORSumII(nums []int) int {
	or := 0
	for _, x := range nums {
		or |= x
	}
	return or << (len(nums) - 1)
}
