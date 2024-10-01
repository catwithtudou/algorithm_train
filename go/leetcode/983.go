package leetcode

func mincostTickets(days []int, costs []int) int {
	lastDay := days[len(days)-1]
	isTravel := make([]bool, lastDay+1)
	for _, v := range days {
		isTravel[v] = true
	}
	memo := make([]int, lastDay+1)
	var dfs func(i int) int
	dfs = func(i int) (res int) {
		if i <= 0 {
			return
		}
		p := &memo[i]
		if *p > 0 {
			return *p
		}
		defer func() {
			*p = res
		}()
		if !isTravel[i] {
			return dfs(i - 1)
		}
		return min(min(dfs(i-1)+costs[0], dfs(i-7)+costs[1]), dfs(i-30)+costs[2])
	}
	return dfs(lastDay)
}
