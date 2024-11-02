package leetcode

func maxEnergyBoost(energyDrinkA []int, energyDrinkB []int) int64 {
	n := len(energyDrinkA)
	c := [2][]int{energyDrinkA, energyDrinkB}
	memo := make([][2]int64, n)
	var dfs func(i, j int) int64
	dfs = func(i, j int) int64 {
		if i < 0 {
			return 0
		}
		p := &memo[i][j]
		if *p == 0 {
			*p = maxInt64(dfs(i-1, j), dfs(i-2, j^1)) + int64(c[j][i])
		}
		return *p

	}
	return maxInt64(dfs(n-1, 0), dfs(n-1, 1))
}
