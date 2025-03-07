package leetcode

import (
	"sort"
)

func beautifulSubsets(nums []int, k int) (ans int) {
	group := make(map[int]map[int]int)
	for _, x := range nums {
		if group[x%k] == nil {
			group[x%k] = make(map[int]int)
		}
		group[x%k][x]++
	}

	ans = 1

	for _, cnt := range group {
		a := []int{}
		for x := range cnt {
			a = append(a, x)
		}
		sort.Ints(a)

		m := len(a)
		f := make([]int, m+1)
		f[0] = 1
		f[1] = 1 << cnt[a[0]]
		for i := 1; i < m; i++ {
			c := cnt[a[i]]
			if a[i]-a[i-1] == k {
				f[i+1] = f[i-1]*(1<<c-1) + f[i]
			} else {
				f[i+1] = f[i] << c
			}
		}

		ans *= f[m]
	}

	return ans - 1
}
