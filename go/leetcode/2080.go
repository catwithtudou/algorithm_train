package leetcode

import "sort"

type RangeFreqQuery map[int][]int

func ConstructorRangeFreqQuery(arr []int) RangeFreqQuery {
	pos := map[int][]int{}
	for i, x := range arr {
		pos[x] = append(pos[x], i)
	}
	return pos
}

func (this RangeFreqQuery) Query(left int, right int, value int) int {
	a := this[value]
	return sort.SearchInts(a, right+1) - sort.SearchInts(a, left)
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * obj := Constructor(arr);
 * param_1 := obj.Query(left,right,value);
 */
