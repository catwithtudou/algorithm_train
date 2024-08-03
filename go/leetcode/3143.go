package leetcode

import (
	"math/bits"
	"sort"

	"golang.org/x/exp/slices"
)

func maxPointsInsideSquare(points [][]int, s string) int {
	m := make(map[int][]uint8)
	lList := make([]int, 0)
	for i, v := range points {
		l := max(abs(v[0]), abs(v[1]))
		if _, ok := m[l]; !ok {
			lList = append(lList, l)
			m[l] = make([]uint8, 0)
		}
		m[l] = append(m[l], s[i]-'a')
	}
	slices.Sort(lList)
	r := make(map[uint8]bool)
	ans := 0
	for _, v := range lList {
		tNum := 0
		for _, t := range m[v] {
			if !r[t] {
				tNum++
				r[t] = true
				continue
			}
			return ans
		}
		ans += tNum
	}

	return ans
}

func maxPointsInsideSquareWithSearch(points [][]int, s string) int {
	ans := 0
	sort.Search(1_000_000_001, func(size int) bool {
		vis := 0
		for i, p := range points {
			if abs(p[0]) <= size && abs(p[1]) <= size {
				c := s[i] - 'a'
				if vis>>c&1 > 0 {
					return true
				}
				vis |= 1 << c
			}
		}
		ans = bits.OnesCount(uint(vis))
		return false
	})
	return ans
}
