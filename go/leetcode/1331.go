package leetcode

import "golang.org/x/exp/slices"

func arrayRankTransform(arr []int) []int {
	sortedArr := slices.Clone(arr)
	slices.Sort(sortedArr)
	sortedArr = slices.Compact(sortedArr)

	for i, x := range arr {
		arr[i] = slices.Index(sortedArr, x) + 1
	}
	return arr
}
