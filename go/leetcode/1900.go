package leetcode

import "math"

func earliestAndLatest(n int, firstPlayer int, secondPlayer int) []int {
	type pair struct{ earliest, latest int }
	memo := make([][][]pair, n+1)
	for i := range memo {
		memo[i] = make([][]pair, n+1)
		for j := range memo[i] {
			memo[i][j] = make([]pair, n+1)
		}
	}

	var dfs func(int, int, int) pair
	dfs = func(n, first, second int) (res pair) {
		if first+second == n+1 {
			return pair{1, 1}
		}

		if first+second > n+1 {
			first, second = n+1-second, n+1-first
		}

		p := &memo[n][first][second]
		if p.earliest > 0 {
			return *p
		}
		defer func() { *p = res }()

		m := (n + 1) / 2
		minMid, maxMid := 0, second-first
		if second > m {
			minMid, maxMid = second-n/2-1, m-first
		}
		res.earliest = math.MaxInt

		for left := range first {
			for mid := minMid; mid < maxMid; mid++ {
				r := dfs(m, left+1, left+mid+2)
				res.earliest = min(res.earliest, r.earliest)
				res.latest = max(res.latest, r.latest)
			}
		}

		res.earliest++
		res.latest++
		return res
	}

	ans := dfs(n, firstPlayer, secondPlayer)
	return []int{ans.earliest, ans.latest}
}
