package leetcode

import (
	"math/bits"
	"slices"
)

func uniqueXorTripletsII(nums []int) (ans int) {
	u := 1 << bits.Len(uint(slices.Max(nums)))

	has := make([]bool, u)
	for i, x := range nums {
		for _, y := range nums[i:] {
			has[x^y] = true
		}
	}

	has3 := make([]bool, u)
	for xy, b := range has {
		if !b {
			continue
		}
		for _, z := range nums {
			has3[xy^z] = true
		}
	}

	for _, v := range has3 {
		if v {
			ans++
		}
	}

	return ans
}
