package leetcode

import (
	"sort"

	"golang.org/x/exp/slices"
)

type data struct {
	a, b int
}

func maximumSumQueries(nums1 []int, nums2 []int, queries [][]int) []int {
	n := len(nums1)
	pairList := make([]data, n)
	for i, v := range nums1 {
		pairList[i] = data{a: v, b: nums2[i]}
	}
	slices.SortFunc(pairList, func(a, b data) int {
		if a.a > b.a {
			return -1
		}
		return 1
	})

	qIdx := make([]int, len(queries))
	for i := range qIdx {
		qIdx[i] = i
	}
	slices.SortFunc(qIdx, func(a, b int) int {
		if queries[a][0] > queries[b][0] {
			return -1
		}
		return 1
	})

	ans := make([]int, len(queries))
	singleStack := make([]data, 0, n)
	j := 0
	for _, i := range qIdx {
		x, y := queries[i][0], queries[i][1]
		for ; j < n && pairList[j].a >= x; j++ {
			for len(singleStack) > 0 && singleStack[len(singleStack)-1].b <= pairList[j].a+pairList[j].b {
				singleStack = singleStack[:len(singleStack)-1]
			}

			if len(singleStack) == 0 || pairList[j].b > singleStack[len(singleStack)-1].a {
				singleStack = append(singleStack, data{pairList[j].b, pairList[j].a + pairList[j].b})
			}
		}

		if idx := sort.Search(len(singleStack), func(i int) bool {
			return singleStack[i].a >= y
		}); idx < len(singleStack) {
			ans[i] = singleStack[idx].b
		} else {
			ans[i] = -1
		}

	}

	return ans
}
