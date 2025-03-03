package leetcode

import "math"

func palindromePartition(s string, k int) int {
	n := len(s)

	changeMemo := make([][]int, n)
	for i := range changeMemo {
		changeMemo[i] = make([]int, n)
		for j := range changeMemo[i] {
			changeMemo[i][j] = -1
		}
	}

	var change func(int, int) int
	change = func(l, r int) (res int) {
		if l >= r {
			return
		}
		p := &changeMemo[l][r]
		if *p != -1 {
			return *p
		}
		defer func() { *p = res }()

		res = change(l+1, r-1)
		if s[l] != s[r] {
			res++
		}

		return
	}

	memo := make([][]int, k)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(i, r int) (res int) {
		if i == 0 {
			return change(0, r)
		}

		p := &memo[i][r]
		if *p != -1 {
			return *p
		}
		defer func() { *p = res }()

		res = math.MaxInt
		for l := i; l <= r; l++ {
			res = min(res, dfs(i-1, l-1)+change(l, r))
		}

		return
	}

	return dfs(k-1, n-1)
}
