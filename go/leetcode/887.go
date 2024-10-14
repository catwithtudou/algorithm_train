package leetcode

func superEggDrop(k, n int) int {
	memo := [][]int{{}}
	var dfs func(int, int) int
	dfs = func(i, j int) int {
		if i == 0 || j == 0 {
			return 0
		}
		p := &memo[i][j]
		if *p != 0 { // 之前计算过
			return *p
		}
		*p = dfs(i-1, j) + dfs(i-1, j-1) + 1
		return *p
	}
	for i := 1; ; i++ {
		memo = append(memo, make([]int, k+1)) // 动态创建 memo
		if dfs(i, k) >= n {
			return i
		}
	}
}
