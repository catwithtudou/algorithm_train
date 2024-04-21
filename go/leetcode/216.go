package leetcode

import "golang.org/x/exp/slices"

func combinationSum3(k int, n int) (ans [][]int) {
	var path []int
	var dfs func(int, int)
	dfs = func(i int, t int) { // i 枚举要选的数，t 需要满足的和
		d := k - len(path)
		if t < 0 || t > (i+i-d+1)*d/2 {
			return
		}
		if d == 0 {
			ans = append(ans, slices.Clone(path))
			return
		}

		for j := i; j >= d; j-- {
			path = append(path, j)
			dfs(j-1, t-j)
			path = path[:len(path)-1]
		}
	}
	dfs(9, n)
	return
}

func combinationSum3216(k int, n int) (ans [][]int) {
	var path []int
	var dfs func(int, int)
	dfs = func(i int, t int) {
		d := k - len(path)
		if t < 0 || t > (i+i-d+1)*d/2 {
			return
		}
		if d == 0 {
			ans = append(ans, slices.Clone(path))
			return
		}

		if i > d {
			dfs(i-1, t)
		}

		path = append(path, i)
		dfs(i-1, t-i)
		path = path[:len(path)-1]

	}
	dfs(9, n)
	return
}
