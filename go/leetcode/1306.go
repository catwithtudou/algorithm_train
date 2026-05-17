package leetcode

func canReach(arr []int, start int) bool {
	n := len(arr)
	vis := make([]bool, n)

	var dfs func(int) bool
	dfs = func(x int) bool {
		if x < 0 || x >= n || vis[x] {
			return false
		}

		vis[x] = true

		if arr[x] == 0 {
			return true
		}

		return dfs(x+arr[x]) || dfs(x-arr[x])
	}

	return dfs(start)
}
