package leetcode

import "slices"

func minimumDeletions(word string, k int) int {
	cnt := make([]int, 26)
	for _, c := range word {
		cnt[c-'a']++
	}
	slices.Sort(cnt)

	maxSave := 0
	for i, base := range cnt {
		sum := 0
		for _, c := range cnt[i:] {
			sum += min(c, base+k)
		}
		maxSave = max(maxSave, sum)
	}
	return len(word) - maxSave
}
