package leetcode

import "golang.org/x/exp/slices"

func minDeletionSizeIII(strs []string) int {

	lessEq := func(j, i int) bool {
		for _, s := range strs {
			if s[j] > s[i] {
				return false
			}
		}
		return true
	}

	m := len(strs[0])
	f := make([]int, m)
	for i := range m {
		for j := range i {
			if f[j] > f[i] && lessEq(j, i) {
				f[i] = f[j]
			}
		}
		f[i]++
	}

	return m - slices.Max(f)
}
