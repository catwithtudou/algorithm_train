package leetcode

import "golang.org/x/exp/slices"

func maximumLength(s string) int {
	group := [26][]int{}
	cnt := 0
	for i := range s {
		cnt++
		if i+1 == len(s) || s[i] != s[i+1] {
			group[s[i]-'a'] = append(group[s[i]-'a'], cnt)
			cnt = 0
		}
	}

	ans := 0
	for _, a := range group {
		if len(a) == 0 {
			continue
		}

		slices.SortFunc(a, func(a, b int) int {
			return b - a
		})
		a = append(a, 0, 0)
		ans = max(ans, a[0]-2)
		ans = max(ans, min(a[0]-1, a[1]))
		ans = max(ans, a[2])
	}
	if ans == 0 {
		return -1
	}

	return ans
}
