package leetcode

import (
	"math/bits"

	"golang.org/x/exp/slices"
)

func pathExistenceQueries3534(n int, nums []int, maxDiff int, queries [][]int) []int {
	idx := make([]int, n)
	for i := range n {
		idx[i] = i
	}
	slices.SortFunc(idx, func(i, j int) int { return nums[i] - nums[j] })

	rank := make([]int, n)
	for i, j := range idx {
		rank[j] = i
	}

	pa := make([][]int, n)
	mx := bits.Len(uint(n))
	left := 0
	for i, j := range idx {
		for nums[j]-nums[idx[left]] > maxDiff {
			left++
		}
		pa[i] = make([]int, mx)
		pa[i][0] = left
	}

	for i := range mx - 1 {
		for x := range pa {
			p := pa[x][i]
			pa[x][i+1] = pa[p][i]
		}
	}

	ans := make([]int, len(queries))

	for qi, q := range queries {
		l, r := q[0], q[1]
		if l == r {
			continue
		}

		l, r = rank[l], rank[r]
		if l > r {
			l, r = r, l
		}

		res := 0
		for i := mx - 1; i >= 0; i-- {
			if pa[r][i] > l {
				res |= 1 << i
				r = pa[r][i]
			}
		}
		if pa[r][0] > l {
			ans[qi] = -1
		} else {
			ans[qi] = res + 1
		}
	}

	return ans
}
