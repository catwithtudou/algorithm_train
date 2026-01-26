package leetcode

import (
	"math"

	"golang.org/x/exp/slices"
)

func minimumAbsDifference(arr []int) (ans [][]int) {
	slices.Sort(arr)
	minDiff := math.MaxInt32
	for i := 1; i < len(arr); i++ {
		diff := arr[i] - arr[i-1]
		if diff < minDiff {
			minDiff = diff
			ans = [][]int{{arr[i-1], arr[i]}}
		} else if diff == minDiff {
			ans = append(ans, []int{arr[i-1], arr[i]})
		}
	}
	return
}
