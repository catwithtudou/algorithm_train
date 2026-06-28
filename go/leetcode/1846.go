package leetcode

import "golang.org/x/exp/slices"

func maximumElementAfterDecrementingAndRearranging(arr []int) int {
	slices.Sort(arr)
	n := len(arr)
	arr[0] = 1
	for i := 1; i < len(arr); i++ {
		arr[i] = min(arr[i], arr[i-1]+1)
	}
	return arr[n-1]
}
