package leetcode

func distance(nums []int) []int64 {
	groups := map[int][]int{}
	for i, x := range nums {
		groups[x] = append(groups[x], i)
	}

	ans := make([]int64, len(nums))
	for _, a := range groups {
		n := len(a)
		s := make([]int, n+1)
		for i, x := range a {
			s[i+1] = s[i] + x // 前缀和
		}
		for i, target := range a {
			left := target*i - s[i]
			right := s[n] - s[i] - target*(n-i)
			ans[target] = int64(left + right)
		}
	}
	return ans
}
