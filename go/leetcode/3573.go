package leetcode

import "math"

func maximumProfit(prices []int, k int) int64 {
	n := len(prices)
	memo := make([][][3]int, n)
	for i := range memo {
		memo[i] = make([][3]int, k+1)
		for j := range memo[i] {
			memo[i][j] = [3]int{math.MinInt, math.MinInt, math.MinInt} // MinInt 表示还没有计算过
		}
	}

	// 在 [0,i] 中完成至多 j 笔交易，第 i 天【结束时】的状态为 endState 的情况下的最大收益
	// 0=未持有股票，1=持有股票，2=做空中
	var dfs func(int, int, int) int
	dfs = func(i, j, endState int) (res int) {
		if j < 0 {
			return math.MinInt / 2 // 防止溢出
		}
		if i < 0 {
			if endState == 1 {
				return math.MinInt / 2
			}
			return
		}
		ptr := &memo[i][j][endState]
		if *ptr != math.MinInt { // 之前计算过
			return *ptr
		}
		defer func() { *ptr = res }() // 记忆化
		p := prices[i]
		if endState == 0 {
			return max(dfs(i-1, j, 0), dfs(i-1, j, 1)+p, dfs(i-1, j, 2)-p)
		}
		if endState == 1 {
			return max(dfs(i-1, j, 1), dfs(i-1, j-1, 0)-p)
		}
		return max(dfs(i-1, j, 2), dfs(i-1, j-1, 0)+p)
	}

	return int64(dfs(n-1, k, 0))
}
