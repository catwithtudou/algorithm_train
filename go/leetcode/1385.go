package leetcode

import (
	"sort"

	"golang.org/x/exp/slices"
)

func findTheDistanceValue(arr1 []int, arr2 []int, d int) (ans int) {

	slices.Sort(arr2)

	for _, x := range arr1 {
		i := sort.SearchInts(arr2, x-d)
		if i == len(arr2) || arr2[i] > x+d {
			ans++
		}
	}

	return
}
