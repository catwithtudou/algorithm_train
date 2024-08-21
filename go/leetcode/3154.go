package leetcode

func waysToReachStair(k int) int {
	type args struct {
		i, j    int
		preDown bool
	}
	memo := map[args]int{}
	var dfs func(int, int, bool) int
	dfs = func(i, j int, preDown bool) int {
		if i > k+1 { // 无法到达终点 k
			return 0
		}
		p := args{i, j, preDown}
		if v, ok := memo[p]; ok { // 之前算过了
			return v
		}
		res := dfs(i+1<<j, j+1, false) // 操作二
		if !preDown && i > 0 {
			res += dfs(i-1, j, true) // 操作一
		}
		if i == k {
			res++
		}
		memo[p] = res // 记忆化
		return res
	}
	return dfs(1, 0, false)
}
