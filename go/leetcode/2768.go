package leetcode

func maxScore2768(nums []int, x int) int64 {
	n := len(nums)
	memo := make([][2]int, n)
	for i := range memo {
		memo[i] = [2]int{-1, -1}
	}

	var dfs func(i, j int) int
	dfs = func(i, j int) (res int) {
		if i == n {
			return
		}

		p := &memo[i][j]
		if *p != -1 {
			return *p
		}
		defer func() { *p = res }()

		if nums[i]%2 != j {
			return dfs(i+1, j)
		}

		return max(dfs(i+1, j), dfs(i+1, j^1)-x) + nums[i]
	}

	return int64(dfs(0, nums[0]%2))
}

func maxScore2768Other(nums []int, x int) int64 {
	n := len(nums)
	f := make([][2]int, n+1)
	for i := n - 1; i >= 0; i-- {
		v := nums[i]
		r := v % 2
		f[i][r^1] = f[i+1][r^1]
		f[i][r] = max(f[i+1][r], f[i+1][r^1]-x) + v
	}

	return int64(f[0][nums[0]%2])
}

func maxScore2768Max(nums []int, x int) int64 {
	f := [2]int{}
	for i := len(nums) - 1; i >= 0; i-- {
		v := nums[i]
		r := v % 2
		f[r] = max(f[r], f[r^1]-x) + v
	}

	return int64(f[nums[0]%2])
}
