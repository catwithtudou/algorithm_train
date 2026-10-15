package leetcode

func sumCounts(nums []int) int {
	n := len(nums)
	res := 0
	for i := 0; i < n; i++ {
		s := make(map[int]struct{})
		for j := i; j < n; j++ {
			s[nums[j]] = struct{}{}
			res += len(s) * len(s)
		}
	}
	return res
}
