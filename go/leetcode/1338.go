package leetcode

import (
	"maps"
	"slices"
)

func minSetSize(arr []int) int {
	freq := make(map[int]int, 0)
	for _, v := range arr {
		freq[v]++
	}

	cnt := slices.SortedFunc(maps.Values(freq), func(i, j int) int {
		return j - i
	})

	nums := 0
	for i, c := range cnt {
		nums += c
		if nums >= len(arr)/2 {
			return i + 1
		}
	}

	return -1
}
