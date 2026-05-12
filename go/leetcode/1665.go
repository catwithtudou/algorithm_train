package leetcode

import "golang.org/x/exp/slices"

func minimumEffort(tasks [][]int) (ans int) {

	slices.SortFunc(tasks, func(a, b []int) int {
		return (b[1] - b[0]) - (a[1] - a[0])
	})

	s := 0

	for _, task := range tasks {
		ac, me := task[0], task[1]
		ans = max(ans, s+me)
		s += ac
	}

	return ans
}
