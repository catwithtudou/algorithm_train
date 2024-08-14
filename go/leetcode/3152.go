package leetcode

func isArraySpecialII(nums []int, queries [][]int) []bool {
	s := make([]int, len(nums))
	for i := 1; i < len(nums); i++ {
		s[i] = s[i-1] + (nums[i]^nums[i-1]^1)&1
	}
	ans := make([]bool, len(queries))
	for i, v := range queries {
		ans[i] = s[v[0]] == s[v[1]]
	}
	return ans
}
