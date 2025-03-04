package leetcode

import "math"

func checkPartitioning(s string) bool {
	return palindromePartitionII(s, 3) == 0
}

func palindromePartitionII(s string, k int) int {
	n := len(s)

	miniChange := make([][]int, n)
	for i := n - 1; i >= 0; i-- {
		miniChange[i] = make([]int, n)
		for j := i + 1; j < n; j++ {
			miniChange[i][j] = miniChange[i+1][j-1]
			if s[i] != s[j] {
				miniChange[i][j]++
			}
		}
	}

	f := miniChange[0]
	for i := 1; i < k; i++ {
		for r := n - k + i; r >= i; r-- {
			f[r] = math.MaxInt
			for l := i; l <= r; l++ {
				f[r] = min(f[r], f[l-1]+miniChange[l][r])
			}
		}
	}

	return f[n-1]
}
