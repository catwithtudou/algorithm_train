package leetcode

func countOfPairsII(nums []int) int {
	const mod = 1_000_000_007
	n := len(nums)
	m := nums[n-1]
	f := make([]int, m+1)
	for i := range f[:min(nums[0], m)+1] {
		f[i] = 1
	}

	for i := 1; i < n; i++ {
		for j := 1; j <= m; j++ {
			f[j] += f[j-1]
		}
		j0 := max(nums[i]-nums[i-1], 0)
		for j := min(m, nums[i]); j >= j0; j-- {
			f[j] = f[j-j0] % mod
		}
		clear(f[:min(j0, m+1)])
	}

	ans := 0
	for _, v := range f {
		ans += v
	}

	return ans % mod
}
