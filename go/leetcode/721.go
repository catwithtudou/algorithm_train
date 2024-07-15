package leetcode

import (
	"golang.org/x/exp/maps"
	"golang.org/x/exp/slices"
)

func accountsMerge(accounts [][]string) [][]string {
	emailToIndex := map[string][]int{}
	for k, v := range accounts {
		for _, email := range v[1:] {
			emailToIndex[email] = append(emailToIndex[email], k)
		}
	}

	vis := make([]bool, len(accounts))
	emailSet := make(map[string]bool)
	var dfs func(int)
	dfs = func(i int) {
		vis[i] = true
		for _, email := range accounts[i][1:] {
			if emailSet[email] {
				continue
			}
			emailSet[email] = true
			for _, j := range emailToIndex[email] {
				if !vis[j] {
					dfs(j)
				}
			}
		}
	}

	ans := make([][]string, 0, len(accounts))
	for i, v := range vis {
		if v {
			continue
		}
		maps.Clear(emailSet)
		dfs(i)
		res := make([]string, 1, len(emailSet)+1)
		res[0] = accounts[i][0]
		for email := range emailSet {
			res = append(res, email)
		}
		slices.Sort(res[1:])
		ans = append(ans, res)
	}
	return ans
}
