package leetcode

import "golang.org/x/exp/slices"

func countOfPairs(nums []int) int {
	const mod = 1_000_000_007
	n := len(nums)
	m := slices.Max(nums)
	f := make([][]int, n)
	for i := range f {
		f[i] = make([]int, m+1)
	}
	for i := 0; i <= nums[0]; i++ {
		f[0][i] = 1
	}

	s := make([]int, m+1)
	for i := 1; i < n; i++ {
		s[0] = f[i-1][0]
		for k := 1; k <= m; k++ {
			s[k] = s[k-1] + f[i-1][k]
		}
		for j := 0; j <= nums[i]; j++ {
			maxK := j + min(0, nums[i-1]-nums[i])
			if maxK >= 0 {
				f[i][j] = s[maxK] % mod
			}
		}
	}
	ans := 0
	for _, v := range f[n-1][:nums[n-1]+1] {
		ans += v
	}
	return ans % mod
}
