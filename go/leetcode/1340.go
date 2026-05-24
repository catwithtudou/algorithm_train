package leetcode

func maxJumps1340(arr []int, d int) (ans int) {

	n := len(arr)
	memo := make([]int, n)

	var dfs func(i int) int

	dfs = func(i int) (res int) {
		p := &memo[i]
		if *p > 0 {
			return *p
		}
		defer func() { *p = res }()

		for j := i - 1; j >= max(i-d, 0) && arr[j] < arr[i]; j-- {
			res = max(res, dfs(j))
		}

		for j := i + 1; j <= min(i+d, n-1) && arr[j] < arr[i]; j++ {
			res = max(res, dfs(j))
		}

		return res + 1
	}

	for i := range n {
		ans = max(ans, dfs(i))
	}

	return
}
