package leetcode

func maxUncrossedLines(nums1 []int, nums2 []int) int {
	n, m := len(nums1), len(nums2)
	f := make([][]int, n+1)
	for i := range f {
		f[i] = make([]int, m+1)
	}
	for i := 1; i <= n; i++ {
		for j := 1; j <= m; j++ {
			f[i][j] = max(f[i-1][j], f[i][j-1])
			if nums1[i-1] == nums2[j-1] {
				f[i][j] = max(f[i][j], f[i-1][j-1]+1)
			}

		}
	}

	return f[n][m]
}
