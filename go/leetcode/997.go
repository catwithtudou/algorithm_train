package leetcode

func findJudge(n int, trust [][]int) int {
	intD := make([]int, n+1)
	outD := make([]int, n+1)
	for _, edge := range trust {
		intD[edge[1]]++
		outD[edge[0]]++
	}

	for i := 1; i <= n; i++ {
		if intD[i] == n-1 && outD[i] == 0 {
			return i
		}
	}

	return -1
}
