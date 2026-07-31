package leetcode

import (
	"sort"
)

func minimumPushesII(word string) (ans int) {
	cnt := [26]int{}
	for _, w := range word {
		cnt[w-'a']++
	}
	sort.Sort(sort.Reverse(sort.IntSlice(cnt[:])))

	for i, c := range cnt {
		ans += c * (i/8 + 1)
	}

	return
}
