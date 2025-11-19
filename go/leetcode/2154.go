package leetcode

import "golang.org/x/exp/slices"

func findFinalValue(nums []int, original int) int {
	slices.Sort(nums)

	for _, num := range nums {
		if num == original {
			original *= 2
		}
	}

	return original
}
