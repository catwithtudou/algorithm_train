package leetcode

func minCostClimbingStairs(cost []int) int {
	f0, f1 := 0, 0
	for i := 1; i < len(cost); i++ {
		f0, f1 = f1, min(f1+cost[i], f0+cost[i-1])
	}
	return f1
}

func minCostClimbingStairsOther(cost []int) int {
	n := len(cost)
	f := make([]int, n+1)
	for i := 2; i <= n; i++ {
		f[i] = min(f[i-1]+cost[i-1], f[i-2]+cost[i-2])
	}
	return f[n]
}

func minCostClimbingStairsDfs(cost []int) int {
	n := len(cost)
	memo := make([]int, n+1)
	for i := range memo {
		memo[i] = -1
	}

	var dfs func(int) int
	dfs = func(i int) int {
		if i <= 1 {
			return 0
		}
		if memo[i] != -1 {
			return memo[i]
		}
		res := min(dfs(i-1)+cost[i-1], dfs(i-2)+cost[i-2])
		memo[i] = res
		return res
	}

	return dfs(n)
}
